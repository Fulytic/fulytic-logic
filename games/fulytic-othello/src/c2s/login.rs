use fulytic_core::{BaseGameLogic, GameC2S, GameS2CQueue, PlayerInfo};

use crate::{s2c::login::OthelloLoginS2C, OthelloGame};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OthelloLoginC2S {
    pub player: PlayerInfo,
}

impl GameC2S for OthelloLoginC2S {
    type T = OthelloGame;

    fn apply_server(self, game: &mut Self::T, queue: &mut GameS2CQueue<Self::T>) {
        // if game.players.len() < OthelloGame::info().max_players.unwrap().get() {
        //     game.players.push(self.player);
        //     queue.push(OthelloLoginS2C::Success(game.clone()));
        // } else {
        //     queue.push(OthelloLoginS2C::PlayerLimitReached);
        // }
    }
}
