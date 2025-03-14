use std::sync::Arc;

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

pub type GameC2SQueue<T> = TypedPlayerBuf<<T as BaseGameLogic>::C2S>;
pub type GameS2CQueue<T> = TypedPlayerBuf<<T as BaseGameLogic>::S2C>;

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

pub struct PlayerBuf {
    pub info: Arc<PlayerInfo>,
    sender: tokio::sync::mpsc::Sender<()>,
    queue: Arc<Mutex<BufQueue>>,
}

impl PlayerBuf {
    pub fn new(
        sender: tokio::sync::mpsc::Sender<()>,
        info: Arc<PlayerInfo>,
        queue: Arc<Mutex<BufQueue>>,
    ) -> Self {
        Self {
            sender,
            info,
            queue,
        }
    }

    pub async fn encode<T: Codec>(&self, item: T) {
        self.queue.lock().await.encode(item);
        let _ = self.sender.send(()).await;
    }

    pub fn into_typed<T: Codec>(self) -> TypedPlayerBuf<T> {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(transparent)]
pub struct TypedPlayerBuf<T: Codec> {
    buf: PlayerBuf,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Codec> TypedPlayerBuf<T> {
    pub fn new(
        sender: tokio::sync::mpsc::Sender<()>,
        info: Arc<PlayerInfo>,
        queue: Arc<Mutex<BufQueue>>,
    ) -> Self {
        Self {
            buf: PlayerBuf::new(sender, info, queue),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn encode(&self, item: T) {
        self.buf.encode(item).await;
    }

    /// # Safety
    /// This function is unsafe because it does not check the type of the item.
    pub async unsafe fn unchecked_encode<L: Codec>(&self, item: L) {
        self.buf.encode(item).await;
    }
}
