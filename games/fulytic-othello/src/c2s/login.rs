use fulytic_core::{BaseGameLogic, GameC2S, GameS2CQueue, PlayerInfo};

use crate::{
    s2c::{login::OthelloLoginS2C, OthelloGameS2C},
    OthelloGame,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OthelloLoginC2S {
    pub player: PlayerInfo,
}

impl GameC2S for OthelloLoginC2S {
    type T = OthelloGame;

    fn apply_server(self, game: &mut Self::T, queue: &mut GameS2CQueue<Self::T>) {
        if let Err(limit) = Self::T::info().check_players(game.players.len()) {
            queue.push(OthelloGameS2C::Login(OthelloLoginS2C::PlayerLimit(limit)));
            return;
        };
        game.players.push(self.player);
        queue.push(OthelloGameS2C::Login(OthelloLoginS2C::Success(
            game.clone(),
        )));
    }
}
