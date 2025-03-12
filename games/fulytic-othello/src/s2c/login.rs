use fulytic_core::{GameC2SQueue, GameS2C};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OthelloSelectCharS2C {
    Success(bool),
    Fail,
}

impl GameS2C for OthelloSelectCharS2C {
    type T = OthelloGame;

    fn apply_client(self, game: &Self::T, _: &mut GameC2SQueue<Self::T>) {
        match self {
            Self::Success(selected_char) => {}
            Self::Fail => {}
        }
    }
}
