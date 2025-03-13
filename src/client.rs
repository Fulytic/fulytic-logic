use fulytic_core::{PlayerInfo, PlayerLimitError};
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
    MissingId,
    MissingPlayerInfo,
    AlreadyStarted,
    LimitError(PlayerLimitError),
    ServerError,
}

#[cfg(test)]
mod tests {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    };

    use fulytic_core::Codec;

    use super::*;

    #[tokio::test]
    async fn hey() {
        let player = PlayerInfo {
            id: Uuid::new_v4(),
            name: "Alice".to_string(),
        };
        let buf = player.encode().unwrap();
        let mut stream = TcpStream::connect("0.0.0.0:30010").await.unwrap();
        stream.write_all(&buf).await.unwrap();
        let c2s = GameJoinC2S {
            player,
            game_uuid: Uuid::new_v4(),
            game_name: "Othello".to_string(),
        };
        let buf = c2s.encode().unwrap();
        println!("writing c2s");
        // TODO: fix unnecessary sleep
        std::thread::sleep(std::time::Duration::from_secs(1));
        stream.write_all(&buf).await.unwrap();
        println!("reading s2c");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let mut buf = Vec::new();
        stream.read_buf(&mut buf).await.unwrap();
        println!("read s2c {:?}", buf);
        let s2c = GameJoinS2C::decode(&buf).unwrap();
        println!("{:?}", s2c);
    }
}
