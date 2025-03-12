use serde::Serialize;

use crate::BaseGameLogic;

pub mod bufqueue;
pub use bufqueue::BufQueue;

pub trait Codec: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {
    fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
    }

    fn encode_into_slice(&self, slice: &mut [u8]) -> Result<usize, bincode::error::EncodeError> {
        bincode::serde::encode_into_slice(self, slice, bincode::config::standard())
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), bincode::error::DecodeError> {
        bincode::serde::decode_from_slice(data, bincode::config::standard())
    }

    fn decode_from_read<R: std::io::Read>(r: &mut R) -> Result<Self, bincode::error::DecodeError> {
        bincode::serde::decode_from_std_read(r, bincode::config::standard())
    }
}

impl<T> Codec for T where T: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

pub trait GameError: std::error::Error + Codec {}

impl<T> GameError for T where T: std::error::Error + Codec {}

pub type GameC2SQueue<T> = BufQueue<<T as BaseGameLogic>::C2S>;
pub type GameS2CQueue<T> = BufQueue<<T as BaseGameLogic>::S2C>;

#[ambassador::delegatable_trait]
pub trait GameC2S: Codec {
    type T: BaseGameLogic;
    fn apply_server(self, game: &Self::T, queue: &mut GameS2CQueue<Self::T>);
}

#[ambassador::delegatable_trait]
pub trait GameS2C: Codec {
    type T: BaseGameLogic;
    fn apply_client(self, game: &Self::T, queue: &mut GameC2SQueue<Self::T>);
}
