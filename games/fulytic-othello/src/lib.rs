use std::num::NonZeroUsize;

use fulytic_core::{get_lang, BaseGameLogic, Lang, PlayerInfo};
use local_fmt::def_local_fmt;
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
pub struct OthelloGame {
    id: Uuid,
    // 0 is black, 1 is white
    players: Vec<PlayerInfo>,
    black: u64,
    white: u64,
}

impl BaseGameLogic for OthelloGame {
    type C2S = c2s::OthelloGameC2S;
    type S2C = s2c::OthelloGameS2C;

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
            players: Vec::new(),
            black: 0x0000000810000000,
            white: 0x0000001008000000,
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn forced_termination(&mut self) {}
}
