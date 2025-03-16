use bevy::prelude::*;

use crate::AppState;

pub mod component;
pub mod system;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), system::build_main_menu);
    }
}
