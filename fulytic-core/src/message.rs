use enum_table::Enumable;
use local_fmt::def_local_fmt;

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
