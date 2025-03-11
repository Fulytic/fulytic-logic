use serde::Serialize;

use crate::BaseGameLogic;

pub mod bufqueue;
pub use bufqueue::BufQueue;

pub trait Codec: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

impl<T> Codec for T where T: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

pub trait GameError: std::error::Error + Codec {}

impl<T> GameError for T where T: std::error::Error + Codec {}

pub type GameC2SQueue<T> = BufQueue<<T as BaseGameLogic>::C2S>;
pub type GameS2CQueue<T> = BufQueue<<T as BaseGameLogic>::S2C>;

#[ambassador::delegatable_trait]
pub trait GameC2S: Codec {
    type T: BaseGameLogic;
    fn apply_server(self, game: &mut Self::T, queue: &mut GameS2CQueue<Self::T>);
}

#[ambassador::delegatable_trait]
pub trait GameS2C: Codec {
    type T: BaseGameLogic;
    fn apply_client(self, game: &mut Self::T, queue: &mut GameC2SQueue<Self::T>);
}
