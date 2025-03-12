use fulytic_core::PlayerInfo;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameJoinC2S {
    pub player: PlayerInfo,
    pub game_uuid: Uuid,
    pub game_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GameJoinS2C {
    RawGameData(#[serde(with = "serde_bytes")] Vec<u8>),
    AlreadyStarted,
}
