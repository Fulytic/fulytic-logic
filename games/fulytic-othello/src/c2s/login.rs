use fulytic_core::{GameC2S, GameS2CQueue, PlayerInfo};

use crate::OthelloGame;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OthelloLoginC2S {
    pub player: PlayerInfo,
}

impl GameC2S for OthelloLoginC2S {
    type T = OthelloGame;

    fn apply_server(self, game: &mut Self::T, queue: &mut GameS2CQueue<Self::T>) {
        todo!()
    }
}
