use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameCreateC2S {
    pub game_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GameCreateS2C {
    GameCreated(Uuid),
    InvalidGameName,
}
