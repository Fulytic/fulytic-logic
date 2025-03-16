use bevy::prelude::*;

use crate::style::{
    get_button_text_style, get_title_text_style, BUTTON_NODE, MAIN_MENU_NODE, NORMAL_BUTTON_COLOR,
    TITLE_NODE,
};

use super::component::{MainMenu, PlayButton, QuitButton};

pub fn build_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Visibility::default(), MAIN_MENU_NODE.clone(), MainMenu {}))
        .with_children(|parent| {
            let mut title_style = get_title_text_style(&asset_server);
            title_style.1 .0 = Srgba::RED.into();
            // Title
            parent.spawn(TITLE_NODE.clone()).with_children(|parent| {
                parent.spawn((
                    Text("Fulytic".to_string()),
                    title_style,
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
            });
            // Play Button
            parent
                .spawn((
                    Button,
                    BUTTON_NODE.clone(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play".to_string()),
                        get_button_text_style(&asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // Settings Button
            parent
                .spawn((
                    Button,
                    BUTTON_NODE.clone(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Settings".to_string()),
                        get_button_text_style(&asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // Quit Button
            parent
                .spawn((
                    Button,
                    BUTTON_NODE.clone(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Quit".to_string()),
                        get_button_text_style(&asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        });
}
