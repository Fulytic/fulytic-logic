use fulytic_core::{ambassador_impl_GameS2C, GameC2SQueue, GameS2C};
use select::OthelloSelectCharS2C;

pub mod select;

#[derive(Debug, serde::Serialize, serde::Deserialize, ambassador::Delegate)]
#[delegate(fulytic_core::GameS2C)]
pub enum OthelloGameS2C {
    Login(OthelloSelectCharS2C),
}
