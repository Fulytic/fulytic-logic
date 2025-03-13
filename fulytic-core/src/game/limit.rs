use serde::{Deserialize, Serialize};

use super::GameInfo;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    thiserror::Error,
)]
pub enum PlayerLimitError {
    #[error("Too few players")]
    TooFew,
    #[error("Too many players")]
    TooMany,
}

impl GameInfo {
    pub fn check_players(&self, players: usize) -> Result<(), PlayerLimitError> {
        if let Some(min) = self.min_players {
            if players < min.get() {
                return Err(PlayerLimitError::TooFew);
            }
        }
        if self.is_ok_max_players(players) {
            Ok(())
        } else {
            Err(PlayerLimitError::TooMany)
        }
    }

    pub fn is_ok_max_players(&self, players: usize) -> bool {
        if let Some(max) = self.max_players {
            if players > max.get() {
                return false;
            }
        }
        true
    }
}
