use ggez::audio;
use ggez::graphics;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::fmt::Debug;

pub struct Assets {
    pub brick_survived: graphics::Image,
    pub brick_touched: graphics::Image,
    pub skateboard_normal: graphics::Image,
    pub skateboard_rebound: graphics::Image,
    pub ball_flying: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
       let brick_survived = graphics::Image::new(ctx, "/brick_survived.png")?;
       let brick_touched = graphics::Image::new(ctx, "/brick_touched.png")?;
       let skateboard_normal = graphics::Image::new(ctx, "/skateboard_normal_1.png")?;
       let skateboard_rebound = graphics::Image::new(ctx, "/skateboard_rebound.png")?;
       let ball_flying = graphics::Image::new(ctx, "/ball_flying.png")?;

        Ok(Assets {
            brick_survived,
            brick_touched,
            skateboard_normal,
            skateboard_rebound,
            ball_flying,
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, center: Point2<f32>, ctx: &mut Context) -> GameResult<()>;
    fn width(&self, ctx: &mut Context) -> f32;
    fn height(&self, ctx: &mut Context) -> f32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str, ctx: &mut Context) -> GameResult<TextSprite> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let mut text = graphics::Text::new(label);
        text.set_font(font, graphics::PxScale::from(26.0));
        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, top_left: Point2<f32>, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.text, graphics::DrawParam {
            dest: top_left,
            .. Default::default()
        })
    }

    fn width(&self, ctx: &mut Context) -> f32 { self.text.width(ctx) }
    fn height(&self, ctx: &mut Context) -> f32 { self.text.height(ctx) }
}