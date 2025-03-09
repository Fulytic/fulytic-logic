use std::num::NonZeroUsize;

use enum_table::Enumable;
use local_fmt::def_local_fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Enumable)]
pub enum Lang {
    EN,
    JA,
}

static LANG: std::sync::RwLock<Lang> = std::sync::RwLock::new(Lang::EN);

pub struct Messages {
    pub hello: &'static str,
}

def_local_fmt!(
    name = MESSAGES,
    lang = Lang,
    message = Messages,
    supplier = || *LANG.read().unwrap(),
    file_type = "toml",
    lang_folder = "langs"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameInfo {
    pub name: &'static str,
    pub desc: &'static str,
    pub min_players: Option<NonZeroUsize>,
    pub max_players: Option<NonZeroUsize>,
}

pub trait BaseGameLogic {
    type GameStartError: std::error::Error;

    fn new() -> Self;
    fn id(&self) -> Uuid;
    fn info(&self) -> GameInfo;
    fn start(&mut self) -> Self::GameStartError;
    fn close();
}
