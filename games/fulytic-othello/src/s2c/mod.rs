use fulytic_core::{ambassador_impl_GameS2C, GameS2C, GameC2SQueue};
use login::OthelloLoginS2C;

pub mod login;

#[derive(Debug, serde::Serialize, serde::Deserialize, ambassador::Delegate)]
#[delegate(fulytic_core::GameS2C)]
pub enum OthelloGameS2C {
    Login(OthelloLoginS2C),
}
