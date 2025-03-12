use fulytic_core::{GameC2S, GameS2CQueue, PlayerInfo};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OthelloSelectCharC2S {
    pub player: PlayerInfo,
    pub is_white: bool,
}

impl GameC2S for OthelloSelectCharC2S {
    type T = OthelloGame;

    fn apply_server(self, game: &Self::T, queue: &mut GameS2CQueue<Self::T>) {}
}
