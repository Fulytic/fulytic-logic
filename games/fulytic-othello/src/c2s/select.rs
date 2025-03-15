use std::sync::Arc;

use fulytic_core::{GameC2S, GameS2CQueue, PlayerInfo};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OthelloSelectCharC2S {
    pub player: PlayerInfo,
    pub is_white: bool,
}

impl GameC2S for OthelloSelectCharC2S {
    type T = OthelloGame;

    async fn apply_server(self, game: Arc<Self::T>, queue: GameS2CQueue<Self::T>) {
        let _ = game;
        let _ = queue;
    }
}
