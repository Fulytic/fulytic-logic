use crate::core::BaseGameLogic;
use uuid::Uuid;

#[derive(Debug, more_convert::EnumName)]
pub enum Game {
    Othello(crate::othello::OthelloGame),
}

impl Game {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Othello(game) => game.id(),
        }
    }

    pub async fn forced_termination(&mut self) {
        match self {
            Self::Othello(game) => game.forced_termination().await,
        }
    }
}
