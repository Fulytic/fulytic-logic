use std::{num::NonZeroUsize, sync::Arc};

use crate::{
    codec::{Codec, GameC2S, GameS2C},
    BufQueue, GameS2CQueue, PlayerInfo,
};
use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: &'static str,
    pub desc: &'static str,
    pub min_players: Option<NonZeroUsize>,
    pub max_players: Option<NonZeroUsize>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    thiserror::Error,
)]
pub enum PlayerLimitError {
    #[error("Too few players")]
    TooFew,
    #[error("Too many players")]
    TooMany,
}

impl GameInfo {
    pub fn check_players(&self, players: usize) -> Result<(), PlayerLimitError> {
        if let Some(min) = self.min_players {
            if players < min.get() {
                return Err(PlayerLimitError::TooFew);
            }
        }
        if let Some(max) = self.max_players {
            if players > max.get() {
                return Err(PlayerLimitError::TooMany);
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait BaseGameLogic: Sized + Send + Sync + 'static {
    type RawGameData: Codec;

    type S2C: GameS2C<T = Self>;
    type C2S: GameC2S<T = Self>;

    fn info() -> GameInfo;

    fn new(id: Uuid) -> Self;
    async fn raw_data(&self) -> Self::RawGameData;
    fn new_with_raw_data(id: Uuid, data: Self::RawGameData) -> Self;

    fn id(&self) -> Uuid;

    async fn join(&self, player: PlayerInfo) -> Result<(), PlayerLimitError>;

    async fn forced_termination(&self);

    fn decode_c2s_packet(
        self: Arc<Self>,
        packet: BytesMut,
        sernder: tokio::sync::mpsc::Sender<()>,
        player: Arc<PlayerInfo>,
        bufqueue: Arc<Mutex<BufQueue>>,
    ) -> bool {
        match Self::C2S::decode(&packet) {
            Ok((ok, _)) => {
                let bufqueue = GameS2CQueue::<Self>::new(sernder, player, bufqueue);
                tokio::spawn(Self::C2S::apply_server(ok, self, bufqueue));
                true
            }
            Err(_) => false,
        }
    }
}
