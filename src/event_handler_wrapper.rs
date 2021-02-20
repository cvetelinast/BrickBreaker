use crate::assets::Assets;
use crate::debug;
use crate::dialogs_handler::{DialogType, DialogsHandler};
use crate::game_settings_entities::{GameSettingsState, InputState, ScoreDetails, ScreenSize};
use crate::game_workflow_state_reducer::{reduce, GameWorkflowIntent, GameWorkflowState};
use crate::gameplay_entities::{Ball, BrickState, BricksWall, GameplayState, Size, Skateboard};
use crate::math::Collision;
use ggez::conf::Conf;
use ggez::event::{self};
use ggez::timer;
use ggez::{Context, GameError, GameResult};
use std::fs::File;
use std::io::prelude::*;

pub const SCORE_FILE_NAME: &str = "score.txt";

pub struct EventHandlerWrapper {
    game_settings_state: GameSettingsState,
    gameplay_state: GameplayState,
    dialogs_handler: DialogsHandler,
    conf: Conf,
}

impl EventHandlerWrapper {
    pub fn new(conf: Conf, assets: Assets, level: i32, max_score: usize) -> Self {
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
            dialogs_handler: DialogsHandler::new(),
            conf: conf,
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
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS)? {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            return match self.game_settings_state.score_details.game_workflow_state {
                GameWorkflowState::NextLevel => self.update_next_level_dialog(ctx),
                GameWorkflowState::GameOver => self.update_game_over_dialog(ctx),
                GameWorkflowState::Play => self.update_play_game(ctx, seconds),
            };
        }
        Ok(())
    }

    pub fn update_next_level_dialog(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    pub fn update_game_over_dialog(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    pub fn update_play_game(&mut self, _ctx: &mut Context, seconds: f32) -> GameResult<()> {
        self.gameplay_state.skateboard.update(
            seconds,
            self.game_settings_state.input.movement,
            self.game_settings_state.screen_size.screen_width,
            &self.game_settings_state.assets,
        );

        self.update_ball(seconds);

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

        if self.gameplay_state.bricks_wall.all_bricks_are_broken() {
            self.on_win();
        }
        Ok(())
    }

    pub fn update_ball(&mut self, seconds: f32) {
        let new_ball_pos = self.gameplay_state.ball.calculate_new_position(
            seconds,
            self.game_settings_state.screen_size.screen_width,
            self.game_settings_state.screen_size.screen_height,
            self.game_settings_state.assets.ball_flying.width() as f32,
            self.game_settings_state.assets.ball_flying.height() as f32,
        );

        if new_ball_pos.y + self.game_settings_state.assets.ball_flying.height() as f32
            > self.game_settings_state.screen_size.screen_height
            && !debug::is_active()
        {
            self.on_game_over();
        } else {
            self.gameplay_state.ball.update(new_ball_pos);
        }
    }

    pub fn key_down_event(&mut self, keycode: event::KeyCode) {
        match self.game_settings_state.score_details.game_workflow_state {
            GameWorkflowState::NextLevel => self.key_down_event_next_level_dialog(keycode),
            GameWorkflowState::GameOver => self.key_down_event_game_over_dialog(keycode),
            GameWorkflowState::Play => match keycode {
                event::KeyCode::Left => self.game_settings_state.input.movement = -1.0,
                event::KeyCode::Right => self.game_settings_state.input.movement = 1.0,
                event::KeyCode::Space => {}
                _ => (),
            },
        };
    }

    pub fn key_down_event_next_level_dialog(&mut self, keycode: event::KeyCode) {
        match keycode {
            event::KeyCode::Space => match reduce(
                self.game_settings_state.score_details.game_workflow_state,
                GameWorkflowIntent::StartGame,
            ) {
                Ok(state) => self.game_settings_state.score_details.game_workflow_state = state,
                Err(e) => println!("State reducer error: {:?} ", e),
            },
            _ => (),
        };
    }

    pub fn key_down_event_game_over_dialog(&mut self, keycode: event::KeyCode) {
        match keycode {
            event::KeyCode::Space => match reduce(
                self.game_settings_state.score_details.game_workflow_state,
                GameWorkflowIntent::GoToHomePage,
            ) {
                Ok(state) => self.game_settings_state.score_details.game_workflow_state = state,
                Err(e) => println!("State reducer error: {:?} ", e),
            },
            _ => (),
        };
    }

    pub fn on_game_over(&mut self) {
        let level = self.game_settings_state.score_details.score.level;
        self.reset(level);
        let max_score = self
            .game_settings_state
            .score_details
            .score
            .max_score_result;
        write_score_to_file(level, max_score);

        match reduce(
            self.game_settings_state.score_details.game_workflow_state,
            GameWorkflowIntent::Lose,
        ) {
            Ok(state) => self.game_settings_state.score_details.game_workflow_state = state,
            Err(e) => println!("State reducer error: {:?} ", e),
        }
    }

    pub fn on_win(&mut self) {
        let level = self.game_settings_state.score_details.score.level + 1;
        self.reset(level);
        let max_score = self
            .game_settings_state
            .score_details
            .score
            .max_score_result;
        write_score_to_file(level, max_score);

        match reduce(
            self.game_settings_state.score_details.game_workflow_state,
            GameWorkflowIntent::Win,
        ) {
            Ok(state) => self.game_settings_state.score_details.game_workflow_state = state,
            Err(e) => println!("State reducer error: {:?} ", e),
        }
    }

    pub fn key_up_event(&mut self, keycode: event::KeyCode) {
        match self.game_settings_state.score_details.game_workflow_state {
            GameWorkflowState::Play => match keycode {
                event::KeyCode::Left | event::KeyCode::Right => {
                    self.game_settings_state.input.movement = 0.0
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.game_settings_state.score_details.game_workflow_state {
            GameWorkflowState::NextLevel => self.draw_next_level_dialog(ctx),
            GameWorkflowState::GameOver => self.draw_game_over_dialog(ctx),
            GameWorkflowState::Play => self.draw_play_game(ctx),
        }
    }

    pub fn draw_next_level_dialog(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dialogs_handler.draw(
            ctx,
            self.game_settings_state.screen_size.screen_width,
            self.game_settings_state.screen_size.screen_height,
            DialogType::NextLevelDialog,
            self.game_settings_state.score_details.score,
        )
    }

    pub fn draw_game_over_dialog(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dialogs_handler.draw(
            ctx,
            self.game_settings_state.screen_size.screen_width,
            self.game_settings_state.screen_size.screen_height,
            DialogType::GameOverDialog,
            self.game_settings_state.score_details.score,
        )
    }

    pub fn draw_play_game(&mut self, ctx: &mut Context) -> GameResult<()> {
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

    pub fn reset(&mut self, level: i32) {
        if self.game_settings_state.score_details.score.score_result
            > self
                .game_settings_state
                .score_details
                .score
                .max_score_result
        {
            self.game_settings_state
                .score_details
                .score
                .max_score_result = self.game_settings_state.score_details.score.score_result;
        }

        self.game_settings_state.score_details.score.score_result = 0;
        self.game_settings_state.score_details.score.level = level;

        self.gameplay_state.bricks_wall.reset_on_game_over();

        let screen_width = self.conf.window_mode.width;
        let screen_height = self.conf.window_mode.height;
        let skateboard_height = self.game_settings_state.assets.skateboard_normal.height() as f32;
        let ball_width = self.game_settings_state.assets.ball_flying.width() as f32;
        let ball_height = self.game_settings_state.assets.ball_flying.height() as f32;

        self.gameplay_state.ball = Ball::new(
            screen_width,
            screen_height,
            skateboard_height,
            ball_width,
            ball_height,
        );

        let skateboard_size = Size {
            height: self.game_settings_state.assets.skateboard_normal.height() as f32,
            width: self.game_settings_state.assets.skateboard_normal.width() as f32,
        };
        self.gameplay_state.skateboard =
            Skateboard::new(skateboard_size, screen_height, screen_width);
    }
}

pub fn write_score_to_file(level: i32, max_score: usize) {
    let s = SCORE_FILE_NAME;
    let mut file = File::create(s).expect("Unable to read file");
    file.set_len(0).expect("Unable to reset the file size");
    write!(file, "{}\n{}", level, max_score)
        .expect("Unable to write the new level and score to the file");
}
