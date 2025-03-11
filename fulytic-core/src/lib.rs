use std::num::NonZeroUsize;

use enum_table::Enumable;
use local_fmt::def_local_fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Enumable)]
pub enum Lang {
    EN,
    JA,
}

static LANG: std::sync::RwLock<Lang> = std::sync::RwLock::new(Lang::EN);

pub fn set_lang(lang: Lang) {
    *LANG.write().unwrap() = lang;
}

pub fn get_lang() -> Lang {
    *LANG.read().unwrap()
}

pub struct CommonMessages {
    pub hello: &'static str,
}

def_local_fmt!(
    name = COMMON_MESSAGES,
    lang = Lang,
    message = CommonMessages,
    supplier = get_lang,
    file_type = "toml",
    lang_folder = "langs"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: &'static str,
    pub desc: &'static str,
    pub min_players: Option<NonZeroUsize>,
    pub max_players: Option<NonZeroUsize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: Uuid,
    pub name: String,
}

pub trait Codec: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

impl<T> Codec for T where T: Send + Sync + Sized + Serialize + serde::de::DeserializeOwned {}

pub trait GameError: std::error::Error + Codec {}

impl<T> GameError for T where T: std::error::Error + Codec {}

pub type GameC2SQueue<T> = Vec<<T as BaseGameLogic>::C2S>;
pub type GameS2CQueue<T> = Vec<<T as BaseGameLogic>::S2C>;

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

pub trait BaseGameLogic: Codec {
    type S2C: GameS2C<T = Self>;
    type C2S: GameC2S<T = Self>;

    fn info() -> GameInfo;

    fn new(id: Uuid) -> Self;
    fn id(&self) -> Uuid;

    fn forced_termination(&mut self);
}
