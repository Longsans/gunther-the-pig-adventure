use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

mod combat;
mod enemy;
mod map;
mod player;
mod sprite_sheets;

const CLEAR_COLOR: Color = Color::BLACK;

pub struct ArcadeGame;

impl Plugin for ArcadeGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .insert_resource(ClearColor(CLEAR_COLOR))
            .add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::from("Camera"));
}
