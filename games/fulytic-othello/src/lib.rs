use std::{
    num::NonZeroUsize,
    sync::atomic::{AtomicU64, Ordering},
};

use fulytic_core::{get_lang, BaseGameLogic, GameJoinS2C, Lang, PlayerInfo};
use local_fmt::def_local_fmt;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod c2s;
pub mod s2c;

pub struct OthelloMessages {
    pub name: &'static str,
    pub desc: &'static str,
}

def_local_fmt!(
    name = OTHELLO_MESSAGES,
    lang = Lang,
    message = OthelloMessages,
    supplier = get_lang,
    file_type = "toml",
    lang_folder = "langs"
);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OthelloGameStat {
    Started {
        black: PlayerInfo,
        white: PlayerInfo,
    },
    Waiting {
        black: Option<PlayerInfo>,
        white: Option<PlayerInfo>,
    },
}

#[derive(Debug)]
pub struct OthelloGame {
    id: Uuid,
    // 0 is black, 1 is white
    stat: RwLock<OthelloGameStat>,
    black: AtomicU64,
    white: AtomicU64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawOthelloGameData {
    stat: OthelloGameStat,
    black: u64,
    white: u64,
}

#[async_trait::async_trait]
impl BaseGameLogic for OthelloGame {
    type RawGameData = RawOthelloGameData;

    type S2C = s2c::OthelloGameS2C;
    type C2S = c2s::OthelloGameC2S;

    fn info() -> fulytic_core::GameInfo {
        fulytic_core::GameInfo {
            name: OTHELLO_MESSAGES.name,
            desc: OTHELLO_MESSAGES.desc,
            min_players: NonZeroUsize::new(1),
            max_players: NonZeroUsize::new(2),
        }
    }

    fn new(id: Uuid) -> Self {
        Self {
            id,
            stat: RwLock::new(OthelloGameStat::Waiting {
                black: None,
                white: None,
            }),
            black: AtomicU64::new(0x0000000810000000),
            white: AtomicU64::new(0x0000001008000000),
        }
    }

    async fn raw_data(&self) -> Self::RawGameData {
        RawOthelloGameData {
            stat: self.stat.read().await.clone(),
            black: self.black.load(Ordering::Relaxed),
            white: self.white.load(Ordering::Relaxed),
        }
    }

    fn new_with_raw_data(id: Uuid, data: Self::RawGameData) -> Self {
        Self {
            id,
            stat: RwLock::new(data.stat),
            black: AtomicU64::new(data.black),
            white: AtomicU64::new(data.white),
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    async fn join(&self, player: PlayerInfo) -> Result<(), GameJoinS2C> {
        match &mut *self.stat.write().await {
            OthelloGameStat::Waiting { black, white } => {
                if black.is_none() {
                    *black = Some(player);
                    Ok(())
                } else if white.is_none() {
                    *white = Some(player);
                    Ok(())
                } else {
                    Err(GameJoinS2C::AlreadyMaxPlayers)
                }
            }
            OthelloGameStat::Started { .. } => Err(GameJoinS2C::AlreadyStarted),
        }
    }

    async fn forced_termination(&self) {}
}
