use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use character::CharacterPlugin;
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
    Physics,
}

pub struct ArcadeGame;

impl Plugin for ArcadeGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(LdtkMapBackendPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(PlayerPlugin)
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
