use crossbeam::atomic::AtomicCell;
use local_fmt::macros::{def_local_fmt, ConvertStr, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConvertStr, EnumIter)]
pub enum Lang {
    EN,
    JA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ConvertStr, EnumIter)]
pub enum GlobalMessageKey {
    Hello,
}

pub static GLOBAL_LANG: AtomicCell<Lang> = AtomicCell::new(Lang::JA);

def_local_fmt!(
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
