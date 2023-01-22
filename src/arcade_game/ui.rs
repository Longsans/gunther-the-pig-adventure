use crate::arcade_game::GameState;
use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::KayakWidgets};

mod components;
pub mod main_menu;
pub mod pause_menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadResource>()
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(startup);
    }
}

#[derive(Default, Resource)]
pub struct PreloadResource {
    images: Vec<Handle<Image>>,
}

fn startup(
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<PreloadResource>,
) {
    font_mapping.set_default(asset_server.load("antiquity.kayak_font"));

    let panel1_image = asset_server.load("main_menu/panel1.png");
    let kayak_image = asset_server.load("main_menu/kayak.png");
    let logo_image = asset_server.load("main_menu/logo.png");
    let button_image = asset_server.load("main_menu/button.png");
    let button_image_hover = asset_server.load("main_menu/button-hover.png");

    preload_resource.images.extend(vec![
        panel1_image.clone(),
        kayak_image.clone(),
        logo_image.clone(),
        button_image.clone(),
        button_image_hover.clone(),
    ]);
}
