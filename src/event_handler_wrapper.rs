use crate::assets::Assets;
use crate::debug;
use crate::game_settings_entities::{
    GameSettingsState, GameWorkflowState, InputState, ScoreDetails, ScreenSize,
};
use crate::gameplay_entities::{Ball, BrickState, BricksWall, GameplayState, Size, Skateboard};
use crate::math::Collision;
use ggez::conf::Conf;
use ggez::event::{self};
use ggez::timer;
use ggez::{Context, GameError, GameResult};

pub struct EventHandlerWrapper {
    game_settings_state: GameSettingsState,
    gameplay_state: GameplayState,
}

impl EventHandlerWrapper {
    pub fn new(conf: &Conf, lines: Vec<String>, assets: Assets) -> Self {
        let level = lines[0].parse::<i32>().unwrap();
        let max_score = lines[1].parse::<usize>().unwrap();

        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        let screen_size = ScreenSize {
            screen_width: screen_width,
            screen_height: screen_height,
        };

        let skateboard_size = Size {
            height: *(&assets.skateboard_normal.height()) as f32,
            width: *(&assets.skateboard_normal.width()) as f32,
        };
        let skateboard_height = skateboard_size.height;

        let brick_height = assets.brick_survived.height() as f32;
        let brick_width = assets.brick_survived.width() as f32;
        let ball_width = assets.ball_flying.width() as f32;
        let ball_height = assets.ball_flying.height() as f32;

        let game_settings_state = GameSettingsState {
            assets: assets,
            input: InputState::default(),
            screen_size: screen_size,
            score_details: ScoreDetails::new(level, max_score),
        };

        let gameplay_state = GameplayState {
            skateboard: Skateboard::new(skateboard_size, screen_height, screen_width),
            bricks_wall: BricksWall::new(screen_width, screen_height, brick_height, brick_width),
            ball: Ball::new(
                screen_width,
                screen_height,
                skateboard_height,
                ball_width,
                ball_height,
            ),
        };

        EventHandlerWrapper {
            game_settings_state: game_settings_state,
            gameplay_state: gameplay_state,
        }
    }

    pub fn handle_collisions(&mut self) -> GameResult {
        let skateboard_rect = self
            .gameplay_state
            .skateboard
            .bounding_rect(&self.game_settings_state.assets);
        let ball_skateboard_collision = self.gameplay_state.ball.collides_rect(skateboard_rect);
        self.gameplay_state.ball.bounce(ball_skateboard_collision);

        for brick in &mut self.gameplay_state.bricks_wall.bricks {
            if !matches!(brick.state, BrickState::Broken) {
                let brick_rect = brick.bounding_rect(&self.game_settings_state.assets);
                let ball_brick_collision = self.gameplay_state.ball.collides_rect(brick_rect);

                match ball_brick_collision {
                    Collision::None => {}
                    _ => {
                        match brick.broke() {
                            Ok(_) => {}
                            _ => {
                                return Err(GameError::EventLoopError(String::from(
                                    "An error with breaking of a brick occurred.",
                                )));
                            }
                        }
                        self.gameplay_state.ball.bounce(ball_brick_collision);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if matches!(
            self.game_settings_state.score_details.game_workflow_state,
            GameWorkflowState::ShowEnd
        ) {
            return Ok(());
        }
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS)? {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.gameplay_state.skateboard.update(
                seconds,
                self.game_settings_state.input.movement,
                self.game_settings_state.screen_size.screen_width,
                &self.game_settings_state.assets,
            );
            self.gameplay_state.ball.update(
                seconds,
                self.game_settings_state.screen_size.screen_width,
                self.game_settings_state.screen_size.screen_height,
                self.game_settings_state.assets.ball_flying.width() as f32,
                self.game_settings_state.assets.ball_flying.height() as f32,
            );

            match self.handle_collisions() {
                Ok(_) => {}
                _ => {
                    return Err(GameError::EventLoopError(String::from(
                        "An error while handling collisions occurred.",
                    )));
                }
            }

            self.game_settings_state.score_details.score.score_result =
                self.gameplay_state.bricks_wall.broken_bricks_count();
        }
        Ok(())
    }

    pub fn key_down_event(&mut self, keycode: event::KeyCode) {
        match keycode {
            event::KeyCode::Left => self.game_settings_state.input.movement = -1.0,
            event::KeyCode::Right => self.game_settings_state.input.movement = 1.0,
            _ => (),
        }
    }

    pub fn key_up_event(&mut self, keycode: event::KeyCode) {
        match keycode {
            event::KeyCode::Left | event::KeyCode::Right => {
                self.game_settings_state.input.movement = 0.0
            }
            _ => (),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game_settings_state
            .score_details
            .draw(ctx, self.game_settings_state.screen_size.screen_width)?;
        self.gameplay_state
            .skateboard
            .draw(ctx, &self.game_settings_state.assets)?;
        self.gameplay_state
            .bricks_wall
            .draw(ctx, &self.game_settings_state.assets)?;
        self.gameplay_state
            .ball
            .draw(ctx, &self.game_settings_state.assets)?;

        if debug::is_active() {
            debug::draw_rect_outline(
                self.gameplay_state
                    .skateboard
                    .bounding_rect(&self.game_settings_state.assets),
                ctx,
            )
            .unwrap();

            for brick in &mut self.gameplay_state.bricks_wall.bricks {
                debug::draw_rect_outline(
                    brick.bounding_rect(&self.game_settings_state.assets),
                    ctx,
                )
                .unwrap();
            }
        }
        Ok(())
    }
}
