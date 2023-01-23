use crate::arcade_game::GameState;
use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::KayakWidgets};

mod components;
pub mod main_menu;
pub mod pause_menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIAssets>()
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(startup);
    }
}

pub const PANEL_INDEX: usize = 0;
// pub const PAUSE_INDEX: usize = 1;
pub const BUTTON_INDEX: usize = 2;
pub const HOVER_BUTTON_INDEX: usize = 3;

#[derive(Default, Resource)]
pub struct UIAssets {
    images: Vec<Handle<Image>>,
}

fn startup(
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<UIAssets>,
) {
    font_mapping.set_default(asset_server.load("weiholmir.kayak_font"));

    let panel_image = asset_server.load("main_menu/panel.png");
    let pause_icon = asset_server.load("main_menu/pause.png");
    let button_image = asset_server.load("main_menu/norm_button.png");
    let button_image_hover = asset_server.load("main_menu/hover_button.png");

    preload_resource.images.extend(vec![
        panel_image.clone(),
        pause_icon.clone(),
        button_image.clone(),
        button_image_hover.clone(),
    ]);
}
