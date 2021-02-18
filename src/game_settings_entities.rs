use crate::assets::{Assets, Sprite, TextSprite};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

#[derive(Debug, Default)]
pub struct InputState {
    pub movement: f32,
}

#[derive(Debug, Default)]
pub struct ScreenSize {
    pub screen_width: f32,
    pub screen_height: f32,
}

pub struct GameSettingsState {
    pub assets: Assets,
    pub input: InputState,
    pub screen_size: ScreenSize,
    pub score_details: ScoreDetails,
}

pub struct ScoreDetails {
    pub game_workflow_state: GameWorkflowState,
    pub score: Score,
}

pub enum GameWorkflowState {
    PlayLevel,
    ShowEnd,
}

pub struct Score {
    pub score_result: usize,
    pub level: i32,
    pub max_score_result: usize,
}

impl ScoreDetails {
    pub fn new(level: i32, max_score: usize) -> Self {
        let score = Score {
            score_result: 0,
            level: level,
            max_score_result: max_score,
        };

        ScoreDetails {
            game_workflow_state: GameWorkflowState::PlayLevel,
            score: score,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, screen_width: f32) -> GameResult<()> {
        let score_text = format!("Score: {}", self.score.score_result);
        let level_text = format!("Level: {}", self.score.level);
        let max_score_text = format!("Max score: {}", self.score.max_score_result);

        let mut score_sprite = Box::new(TextSprite::new(&score_text, ctx)?);
        let mut level_sprite = Box::new(TextSprite::new(&level_text, ctx)?);
        let mut max_score_sprite = Box::new(TextSprite::new(&max_score_text, ctx)?);

        let score_pos = Point2 { x: 0.0, y: 0.0 };
        let level_pos = Point2 {
            x: (screen_width / 2.0) - level_sprite.width(ctx) / 2.0,
            y: 0.0,
        };
        let max_score_pos = Point2 {
            x: screen_width - max_score_sprite.width(ctx),
            y: 0.0,
        };

        score_sprite.draw(score_pos, ctx)?;
        level_sprite.draw(level_pos, ctx)?;
        max_score_sprite.draw(max_score_pos, ctx)?;

        Ok(())
    }
}
