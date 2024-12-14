use crossbeam::atomic::AtomicCell;
use local_fmt::macros::{def_local_fmt, UseLocalFmt};
use std::num::NonZeroUsize;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, UseLocalFmt)]
pub enum Lang {
    EN,
    JA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, UseLocalFmt)]
pub enum GlobalMessageKey {
    Hello,
}

pub static GLOBAL_LANG: AtomicCell<Lang> = AtomicCell::new(Lang::JA);

def_local_fmt!(
    visibility = pub,
    app_file = "core.toml",
    locales_path = "../locales",
    ident = GLOBAL_MESSAGE,
    lang = Lang,
    key = GlobalMessageKey,
    global = || GLOBAL_LANG.load()
);

#[test]
fn test_global_message() {
    // initialize
    let _ = &*GLOBAL_MESSAGE;
}

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
