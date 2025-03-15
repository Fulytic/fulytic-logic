use fulytic_core::{GameC2SQueue, GameS2C};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OthelloSelectCharS2C {
    Success(bool),
    Fail,
}

impl GameS2C for OthelloSelectCharS2C {
    type T = OthelloGame;

    async fn apply_client(self, game: &Self::T, queue: &mut GameC2SQueue<Self::T>) {
        let _ = game;
        let _ = queue;
    }
}
