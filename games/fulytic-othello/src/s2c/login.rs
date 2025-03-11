use fulytic_core::{GameC2SQueue, GameS2C};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OthelloLoginS2C {
    Success(OthelloGame),
    PlayerLimitReached,
}

impl GameS2C for OthelloLoginS2C {
    type T = OthelloGame;

    fn apply_client(self, game: &mut Self::T, _: &mut GameC2SQueue<Self::T>) {
        match self {
            Self::Success(latest_game) => {
                let _ = std::mem::replace(game, latest_game);
            }
            Self::PlayerLimitReached => {}
        }
    }
}
