use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use rigid_body::RigidBodyPlugin;

mod combat;
mod enemy;
mod map;
mod player;
mod rigid_body;
mod sprite_sheets;

const CLEAR_COLOR: Color = Color::BLACK;

#[derive(SystemLabel)]
enum GameSystem {
    Input,
    Physics,
}

pub struct ArcadeGame;

impl Plugin for ArcadeGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(RigidBodyPlugin)
            .insert_resource(ClearColor(CLEAR_COLOR))
            .add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::from("Camera"));
}
