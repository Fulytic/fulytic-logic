use crate::PlayerInfo;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameJoinC2S {
    pub player: PlayerInfo,
    pub game_uuid: Uuid,
    pub game_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GameJoinS2C {
    RawGameData {
        game_uuid: Uuid,
        game_name: String,
        #[serde(with = "serde_bytes")]
        raw_data: Vec<u8>,
    },
    MissingGameId,
    MissingPlayerInfo,
    AlreadyStarted,
    AlreadyMaxPlayers,
    ServerError,
}
