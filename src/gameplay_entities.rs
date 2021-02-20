use ggez::graphics;
use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameError, GameResult};

use crate::assets::Assets;
use crate::math::{circle_collides_rect, Collision};

pub struct GameplayState {
    pub skateboard: Skateboard,
    pub bricks_wall: BricksWall,
    pub ball: Ball,
}

#[derive(Debug)]
pub struct Size {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug)]
pub struct Ball {
    pub state: BallState,
    pub pos: Point2<f32>,
    pub direction: Point2<f32>,
    pub radius: f32,
}

#[derive(Debug)]
pub enum BallState {
    Flying,
    Crashing,
}

impl Ball {
    pub const SPEED: f32 = 350.0;

    pub fn new(
        screen_width: f32,
        screen_height: f32,
        skateboard_height: f32,
        ball_width: f32,
        ball_height: f32,
    ) -> Self {
        let ball_pos = Point2 {
            x: screen_width / 2.0 - ball_width / 2.0,
            y: screen_height - skateboard_height - ball_height,
        };

        return Ball {
            state: BallState::Flying,
            pos: ball_pos,
            direction: Point2 { x: 1.0, y: 1.0 },
            radius: ball_width / 2.0,
        };
    }

    pub fn update(&mut self, point2: Point2<f32>) {
        self.pos.x = point2.x;
        self.pos.y = point2.y;
    }

    pub fn calculate_new_position(
        &mut self,
        seconds: f32,
        screen_width: f32,
        screen_height: f32,
        ball_width: f32,
        ball_height: f32,
    ) -> Point2<f32> {
        let new_x = self.pos.x + Self::SPEED * seconds * self.direction.x;
        let new_y = self.pos.y + Self::SPEED * seconds * self.direction.y;

        if new_x + ball_width > screen_width || new_x < 0.0 {
            self.direction.x = -self.direction.x;
        }

        if new_y + ball_height > screen_height || new_y < 0.0 {
            self.direction.y = -self.direction.y;
        }
        Point2 { x: new_x, y: new_y }
    }

    pub fn bounce(&mut self, collision: Collision) {
        match collision {
            Collision::Right | Collision::Left => {
                self.direction.x = -self.direction.x;
            }
            Collision::Top | Collision::Bottom => {
                self.direction.y = -self.direction.y;
            }
            Collision::None => {}
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        match self.state {
            BallState::Flying => {
                graphics::draw(
                    ctx,
                    &assets.ball_flying,
                    graphics::DrawParam {
                        dest: self.pos,
                        scale: Vector2 { x: 1.0, y: 1.0 },
                        offset: Point2 { x: 0.0, y: 0.0 },
                        ..Default::default()
                    },
                )?;
            }

            BallState::Crashing => {
                graphics::draw(
                    ctx,
                    &assets.ball_flying,
                    graphics::DrawParam {
                        dest: self.pos,
                        scale: Vector2 { x: 1.0, y: 1.0 },
                        offset: Point2 { x: 0.0, y: 0.0 },
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    pub fn collides_rect(&self, bounding_box: graphics::Rect) -> Collision {
        circle_collides_rect(
            self.pos.x + self.radius,
            self.pos.y + self.radius,
            self.radius,
            bounding_box.x,
            bounding_box.y,
            bounding_box.w,
            bounding_box.h,
        )
    }
}

#[derive(Debug)]
pub struct BricksWall {
    pub bricks: Vec<Brick>,
}

impl BricksWall {
    pub const PADDING: f32 = 15.0;
    pub const BRICK_PADDING: f32 = 10.0;
    pub const BRICKS_PERCENTAGE_OF_SCREEN_HEIGHT: f32 = 0.6;

    pub fn new(screen_width: f32, screen_height: f32, brick_height: f32, brick_width: f32) -> Self {
        let mut bricks = Vec::new();

        // Horizontal calculations
        let brick_horizontal_size = brick_width + 2.0 * Self::BRICK_PADDING;
        let left_horizontal_space = screen_width - 2.0 * Self::PADDING;
        let bricks_columns_count = (left_horizontal_space / brick_horizontal_size) as i32;
        let left_horizontal_space_after_bricks =
            left_horizontal_space - (brick_horizontal_size * (bricks_columns_count as f32));
        let offset_from_left = left_horizontal_space_after_bricks / 2.0;

        // Vertical calculations
        let brick_vertical_size = brick_height + 2.0 * Self::BRICK_PADDING;
        let left_vertical_space = screen_height * Self::BRICKS_PERCENTAGE_OF_SCREEN_HEIGHT;
        let bricks_rows_count = (left_vertical_space / brick_vertical_size) as i32;

        for i in 0..bricks_columns_count {
            for j in 0..bricks_rows_count {
                let x = Self::PADDING
                    + offset_from_left
                    + Self::BRICK_PADDING
                    + (i as f32) * brick_horizontal_size;
                let y =
                    Self::PADDING + (brick_vertical_size / 2.0) + (j as f32) * brick_vertical_size;
                let brick = Brick {
                    state: BrickState::Survived,
                    pos: Point2 { x, y },
                };
                bricks.push(brick);
            }
        }

        BricksWall { bricks }
    }

    pub fn reset_on_game_over(&mut self) {
        for brick in &mut self.bricks {
            brick.reset();
        }
    }

    pub fn all_bricks_are_broken(&mut self) -> bool {
        self.broken_bricks_count() == self.bricks.len()
    }

    pub fn update() {}
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        for brick in &self.bricks {
            match brick.draw(ctx, assets) {
                Ok(_) => {}
                _ => {
                    return Err(GameError::EventLoopError(String::from(
                        "An error with drawing of a brick occurred.",
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn broken_bricks_count(&self) -> usize {
        *&self
            .bricks
            .iter()
            .filter(|&brick| matches!(brick.state, BrickState::Broken))
            .count()
    }
}

impl Brick {
    fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        match self.state {
            BrickState::Survived => graphics::draw(
                ctx,
                &assets.brick_survived,
                graphics::DrawParam {
                    dest: self.pos,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                    ..Default::default()
                },
            ),

            BrickState::Touched => graphics::draw(
                ctx,
                &assets.brick_touched,
                graphics::DrawParam {
                    dest: self.pos,
                    scale: Vector2 { x: 1.0, y: 1.0 },
                    offset: Point2 { x: 0.0, y: 0.0 },
                    ..Default::default()
                },
            ),

            BrickState::Broken => Ok(()),
        }
    }

    pub fn bounding_rect(&self, assets: &Assets) -> graphics::Rect {
        let sprite = match self.state {
            BrickState::Survived => &assets.brick_survived,
            BrickState::Touched => &assets.brick_touched,
            BrickState::Broken => &assets.brick_touched, // Fake
        };

        let left = self.pos.x;
        let right = self.pos.x + (sprite.width() as f32);
        let top = self.pos.y;
        let bottom = self.pos.y + (sprite.height() as f32);

        graphics::Rect::new(left, top, right - left, bottom - top)
    }

    pub fn broke(&mut self) -> GameResult<()> {
        match self.state {
            BrickState::Survived => {
                self.state = BrickState::Touched;
            }
            BrickState::Touched => {
                self.state = BrickState::Broken;
            }
            BrickState::Broken => {
                self.state = BrickState::Broken;
            }
        };
        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = BrickState::Survived;
    }
}

#[derive(Debug)]
pub struct Brick {
    pub state: BrickState,
    pub pos: Point2<f32>,
}

#[derive(Debug)]
pub enum BrickState {
    Survived,
    Touched,
    Broken,
}

#[derive(Debug)]
pub enum SkateboardState {
    Normal,
    Rebound,
}

#[derive(Debug)]
pub struct Skateboard {
    pub state: SkateboardState,
    pub pos: Point2<f32>,
    velocity: Vector2<f32>,
}

impl Skateboard {
    pub const SPEED: f32 = 600.0;

    pub fn new(asset_size: Size, max_down: f32, max_right: f32) -> Self {
        let pos = Point2 {
            x: (max_right / 2.0) - (asset_size.width / 2.0),
            y: max_down - asset_size.height,
        };

        Skateboard {
            state: SkateboardState::Normal,
            pos: pos,
            velocity: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn get_current_sprite_size(&self, assets: &Assets) -> Size {
        let asset = match self.state {
            SkateboardState::Normal => &assets.skateboard_normal,
            SkateboardState::Rebound => &assets.skateboard_rebound,
        };
        Size {
            height: asset.height() as f32,
            width: asset.width() as f32,
        }
    }

    pub fn update(&mut self, seconds: f32, amount: f32, max_right: f32, assets: &Assets) {
        let current_sprite_size = self.get_current_sprite_size(assets);
        let new_pos = self.pos.x + Self::SPEED * seconds * amount;
        self.pos.x = nalgebra::clamp(new_pos, 0.0, max_right - current_sprite_size.width);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let current_sprite = match self.state {
            SkateboardState::Normal => &assets.skateboard_normal,
            SkateboardState::Rebound => &assets.skateboard_rebound,
        };

        graphics::draw(
            ctx,
            current_sprite,
            graphics::DrawParam {
                dest: self.pos,
                ..Default::default()
            },
        )
    }

    pub fn bounding_rect(&self, assets: &Assets) -> graphics::Rect {
        let sprite_size = self.get_current_sprite_size(assets);
        let left = self.pos.x;
        let right = self.pos.x + (sprite_size.width as f32);
        let top = self.pos.y;
        let bottom = self.pos.y + (sprite_size.height as f32);

        graphics::Rect::new(left, top, right - left, bottom - top)
    }
}
