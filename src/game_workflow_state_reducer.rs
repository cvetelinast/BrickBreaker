use ggez::{GameError, GameResult};
use std::clone::Clone;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum GameWorkflowState {
    NextLevel,
    Play,
    GameOver,
}

#[derive(Debug, Copy, Clone)]
pub enum GameWorkflowIntent {
    StartGame,
    Lose,
    Win,
    GoToHomePage,
}

impl fmt::Display for GameWorkflowState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn reduce(
    state: GameWorkflowState,
    intent: GameWorkflowIntent,
) -> GameResult<GameWorkflowState> {
    match intent {
        GameWorkflowIntent::StartGame => reduce_start_game_intent(state),
        GameWorkflowIntent::Lose => reduce_lose_intent(state),
        GameWorkflowIntent::Win => reduce_win_intent(state),
        GameWorkflowIntent::GoToHomePage => reduce_go_to_home_page_intent(state),
    }
}

pub fn reduce_start_game_intent(state: GameWorkflowState) -> GameResult<GameWorkflowState> {
    match state {
        GameWorkflowState::NextLevel => return Ok(GameWorkflowState::Play),
        other => {
            return Err(GameError::EventLoopError(String::from(format!(
                "Not allowed transition from {} to {} with StartGame intent",
                state, other
            ))))
        }
    }
}

pub fn reduce_lose_intent(state: GameWorkflowState) -> GameResult<GameWorkflowState> {
    match state {
        GameWorkflowState::Play => return Ok(GameWorkflowState::GameOver),
        other => {
            return Err(GameError::EventLoopError(String::from(format!(
                "Not allowed transition from {} to {} with Lose intent",
                state, other
            ))))
        }
    }
}

pub fn reduce_win_intent(state: GameWorkflowState) -> GameResult<GameWorkflowState> {
    match state {
        GameWorkflowState::Play => return Ok(GameWorkflowState::NextLevel),
        other => {
            return Err(GameError::EventLoopError(String::from(format!(
                "Not allowed transition from {} to {} with Win intent",
                state, other
            ))))
        }
    }
}

pub fn reduce_go_to_home_page_intent(state: GameWorkflowState) -> GameResult<GameWorkflowState> {
    match state {
        GameWorkflowState::GameOver => return Ok(GameWorkflowState::NextLevel),
        other => {
            return Err(GameError::EventLoopError(String::from(format!(
                "Not allowed transition from {} to {} with GoToHomePage intent",
                state, other
            ))))
        }
    }
}
