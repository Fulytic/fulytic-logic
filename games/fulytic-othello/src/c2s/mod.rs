use fulytic_core::{ambassador_impl_GameC2S, GameC2S, GameS2CQueue};
use login::OthelloSelectCharC2S;

pub mod login;

#[derive(Debug, serde::Serialize, serde::Deserialize, ambassador::Delegate)]
#[delegate(fulytic_core::GameC2S)]
pub enum OthelloGameC2S {
    SelectChar(OthelloSelectCharC2S),
}
