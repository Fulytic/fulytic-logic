use fulytic_core::PlayerInfo;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameJoinC2S {
    player: PlayerInfo,
    game_uuid: Uuid,
    game_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GameJoinS2C {
    RawGameData(#[serde(with = "serde_bytes")] Vec<u8>),
    AlreadyStarted,
}
