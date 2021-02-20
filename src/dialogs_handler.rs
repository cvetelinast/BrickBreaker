use crate::assets::{Sprite, TextSprite};
use crate::game_settings_entities::Score;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

#[derive(Debug)]
pub struct DialogsHandler {}

pub enum DialogType {
    NextLevelDialog,
    GameOverDialog,
}

impl DialogsHandler {
    pub const PADDING: f32 = 15.0;
    pub fn new() -> Self {
        DialogsHandler {}
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        screen_width: f32,
        screen_height: f32,
        dialog_type: DialogType,
        score: Score,
    ) -> GameResult<()> {
        match dialog_type {
            DialogType::GameOverDialog => {
                self.draw_game_over_dialog(ctx, screen_width, screen_height, score)
            }
            DialogType::NextLevelDialog => {
                self.draw_next_level_dialog(ctx, screen_width, screen_height, score)
            }
        }
    }

    pub fn draw_game_over_dialog(
        &mut self,
        ctx: &mut Context,
        screen_width: f32,
        screen_height: f32,
        score: Score,
    ) -> GameResult<()> {
        let game_over_text = "Game over!";
        let score_text = format!("Score: {}", score.score_result);
        let play_instructions_text = "Press SPACE to go to the home page.";

        let mut game_over_sprite = Box::new(TextSprite::new(&game_over_text, ctx)?);
        let mut score_sprite = Box::new(TextSprite::new(&score_text, ctx)?);
        let mut play_instructions_sprite = Box::new(TextSprite::new(&play_instructions_text, ctx)?);

        let game_over_pos = Point2 {
            x: (screen_width / 2.0) - game_over_sprite.width(ctx) / 2.0,
            y: (screen_height / 2.0) - game_over_sprite.height(ctx) / 2.0,
        };

        let score_pos = Point2 {
            x: (screen_width / 2.0) - score_sprite.width(ctx) / 2.0,
            y: game_over_pos.y + Self::PADDING + score_sprite.height(ctx) / 2.0,
        };

        let play_instructions_pos = Point2 {
            x: (screen_width / 2.0) - play_instructions_sprite.width(ctx) / 2.0,
            y: score_pos.y + Self::PADDING + play_instructions_sprite.height(ctx) / 2.0,
        };
        game_over_sprite.draw(game_over_pos, ctx)?;
        score_sprite.draw(score_pos, ctx)?;
        play_instructions_sprite.draw(play_instructions_pos, ctx)?;

        Ok(())
    }

    pub fn draw_next_level_dialog(
        &mut self,
        ctx: &mut Context,
        screen_width: f32,
        screen_height: f32,
        score: Score,
    ) -> GameResult<()> {
        let level_text = format!("Level: {}", score.level);
        let max_score_text = format!("Max score: {}", score.max_score_result);
        let play_instructions_text = "Press SPACE to play a new game.";

        let mut level_sprite = Box::new(TextSprite::new(&level_text, ctx)?);
        let mut max_score_sprite = Box::new(TextSprite::new(&max_score_text, ctx)?);
        let mut play_instructions_sprite = Box::new(TextSprite::new(&play_instructions_text, ctx)?);

        let level_pos = Point2 {
            x: (screen_width / 2.0) - level_sprite.width(ctx) / 2.0,
            y: (screen_height / 2.0) - level_sprite.height(ctx) / 2.0,
        };

        let max_score_pos = Point2 {
            x: (screen_width / 2.0) - max_score_sprite.width(ctx) / 2.0,
            y: level_pos.y + Self::PADDING + max_score_sprite.height(ctx) / 2.0,
        };

        let play_instructions_pos = Point2 {
            x: (screen_width / 2.0) - play_instructions_sprite.width(ctx) / 2.0,
            y: max_score_pos.y + Self::PADDING + play_instructions_sprite.height(ctx) / 2.0,
        };
        level_sprite.draw(level_pos, ctx)?;
        max_score_sprite.draw(max_score_pos, ctx)?;
        play_instructions_sprite.draw(play_instructions_pos, ctx)?;

        Ok(())
    }
}
