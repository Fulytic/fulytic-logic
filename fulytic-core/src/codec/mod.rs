use std::sync::Arc;

use bufqueue::TypedBufQueue;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::{BaseGameLogic, PlayerInfo};

pub mod bufqueue;
pub use bufqueue::BufQueue;

pub trait Codec: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {
    fn encode(&self) -> Option<Vec<u8>> {
        match bincode::serde::encode_to_vec(self, bincode::config::standard()) {
            Ok(data) => Some(data),
            Err(err) => {
                if let bincode::error::EncodeError::UnexpectedEnd = err {
                    log::error!("encode UnexpectedEnd");
                }
                None
            }
        }
    }

    fn encode_into_slice(&self, slice: &mut [u8]) -> Option<usize> {
        match bincode::serde::encode_into_slice(self, slice, bincode::config::standard()) {
            Ok(cnt) => Some(cnt),
            Err(err) => {
                if let bincode::error::EncodeError::UnexpectedEnd = err {
                    log::error!("encode UnexpectedEnd");
                }
                None
            }
        }
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), bool> {
        match bincode::serde::decode_from_slice(data, bincode::config::standard()) {
            Ok(value) => Ok(value),
            Err(err) => {
                if let bincode::error::DecodeError::UnexpectedEnd { .. } = err {
                    Err(false)
                } else {
                    Err(true)
                }
            }
        }
    }

    fn decode_from_read<R: std::io::Read>(r: &mut R) -> Result<Self, bool> {
        match bincode::serde::decode_from_std_read(r, bincode::config::standard()) {
            Ok(value) => Ok(value),
            Err(err) => {
                if let bincode::error::DecodeError::UnexpectedEnd { .. } = err {
                    Err(false)
                } else {
                    Err(true)
                }
            }
        }
    }
}

impl<T> Codec for T where T: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

pub trait GameError: std::error::Error + Codec {}

impl<T> GameError for T where T: std::error::Error + Codec {}

pub struct PlayerBuf<T: Codec> {
    pub info: Arc<PlayerInfo>,
    sender: tokio::sync::mpsc::Sender<()>,
    queue: Arc<Mutex<TypedBufQueue<T>>>,
}

impl<T: Codec> PlayerBuf<T> {
    pub fn new(
        sender: tokio::sync::mpsc::Sender<()>,
        info: Arc<PlayerInfo>,
        bufqueue: Arc<Mutex<BufQueue>>,
    ) -> Self {
        Self {
            sender,
            info,
            queue: unsafe {
                std::mem::transmute::<Arc<Mutex<BufQueue>>, Arc<Mutex<TypedBufQueue<T>>>>(bufqueue)
            },
        }
    }

    pub async fn encode(&self, item: T) {
        self.queue.lock().await.encode(item);
        let _ = self.sender.send(()).await;
    }
}

pub type GameC2SQueue<T> = PlayerBuf<<T as BaseGameLogic>::C2S>;
pub type GameS2CQueue<T> = PlayerBuf<<T as BaseGameLogic>::S2C>;

#[ambassador::delegatable_trait]
pub trait GameC2S: Codec {
    type T: BaseGameLogic;
    fn apply_server(
        self,
        game: std::sync::Arc<Self::T>,
        queue: GameS2CQueue<Self::T>,
    ) -> impl std::future::Future<Output = ()> + std::marker::Sync + std::marker::Send + 'static;
}

#[ambassador::delegatable_trait]
#[allow(async_fn_in_trait)]
pub trait GameS2C: Codec {
    type T: BaseGameLogic;
    async fn apply_client(self, game: &Self::T, queue: &mut GameC2SQueue<Self::T>);
}
