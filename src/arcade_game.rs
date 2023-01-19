use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use combat::CombatPlugin;
use ldtk::LdtkMapBackendPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

mod character;
mod combat;
mod enemy;
mod ldtk;
mod map;
mod physics;
mod player;

const CLEAR_COLOR: Color = Color::BLACK;

#[derive(SystemLabel)]
#[allow(dead_code)]
enum GameSystem {
    Input,
    Movement,
    Combat,
}

pub struct ArcadeGame;

impl Plugin for ArcadeGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin)
            .add_plugin(LdtkMapBackendPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(PhysicsPlugin)
            .insert_resource(ClearColor(CLEAR_COLOR))
            .add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            projection: OrthographicProjection {
                scale: 0.25,
                ..default()
            },
            ..default()
        })
        .insert(Name::from("Camera"));
}
