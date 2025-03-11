pub use fulytic_core as core;
pub use fulytic_macros as macros;
pub use fulytic_othello as othello;

#[derive(Debug, more_convert::EnumName)]
pub enum Game {
    Othello(othello::OthelloGame),
}
