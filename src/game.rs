use std::sync::Arc;

use crate::core::{BaseGameLogic, PlayerInfo, bufqueue::BufQueue};
use bytes::BytesMut;
use fulytic_core::{Codec, GameJoinS2C};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, more_convert::EnumName)]
#[enum_name(without_trait)]
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

    pub fn decode_c2s_packet(
        &self,
        packet: BytesMut,
        sernder: tokio::sync::mpsc::Sender<()>,
        player: Arc<PlayerInfo>,
        bufqueue: Arc<Mutex<BufQueue>>,
    ) -> bool {
        forwarding!(self, game => game.clone().decode_c2s_packet(packet,sernder, player,bufqueue))
    }
}
