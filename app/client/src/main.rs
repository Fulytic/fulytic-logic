use bevy::prelude::*;
use main_menu::MainMenuPlugin;

pub mod main_menu;
pub mod style;
pub mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_systems(Startup, system::spawn_camera)
        .add_plugins(MainMenuPlugin)
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    CreatingGame,
    JoiningGame,
    InGame,
}
