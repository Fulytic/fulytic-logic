use std::sync::Arc;

use crate::core::{BaseGameLogic, PlayerInfo};
use bytes::BytesMut;
use fulytic_core::{Codec, GameJoinS2C, PlayerBuf};
use uuid::Uuid;

#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[derive(Debug, Clone)]
pub enum Game {
    Othello(Arc<crate::othello::OthelloGame>),
}

macro_rules! forwarding {
    ($var: expr, $game: pat => $action: expr) => {
        match $var {
            Game::Othello($game) => $action,
        }
    };
}

impl Game {
    pub fn new(name: &str, id: Uuid) -> Option<Self> {
        let game = match name {
            "othello" => Game::Othello(Arc::new(crate::othello::OthelloGame::new(id))),
            _ => return None,
        };

        Some(game)
    }

    pub fn id(&self) -> Uuid {
        forwarding!(self, game => game.id())
    }

    pub async fn join(&self, player: PlayerInfo) -> Result<(), GameJoinS2C> {
        forwarding!(self, game => game.join(player).await)
    }

    pub async fn raw_data(&self) -> Option<Vec<u8>> {
        forwarding!(self, game => game.raw_data().await.encode())
    }

    pub async fn forced_termination(&mut self) {
        forwarding!(self, game => game.forced_termination().await)
    }

    pub fn decode_c2s_packet(&self, packet: BytesMut, buf: PlayerBuf) -> bool {
        forwarding!(self, game => game.clone().decode_c2s_packet(packet,buf))
    }
}
