use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use combat::CombatPlugin;
use iyes_loopless::prelude::*;
use ldtk::LdtkMapBackendPlugin;
use physics::{events::*, PhysicsPlugin};
use player::PlayerPlugin;
use ui::{main_menu, pause_menu, UIPlugin};

mod character;
mod combat;
mod enemy;
mod ldtk;
mod map;
mod physics;
mod player;
mod ui;

pub struct ArcadeGame;

impl Plugin for ArcadeGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .insert_resource(MapLevel::default())
            .add_loopless_state(GameState::MainMenu)
            .add_plugin(UIPlugin)
            .add_plugin(LdtkMapBackendPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(PhysicsPlugin)
            .add_event::<CleanupMapEvent>()
            .add_event::<SetupMapEvent>()
            .add_startup_system(spawn_camera)
            //
            // map -> main menu transition
            .add_enter_system(GameState::InGame, ldtk::setup.run_if(setup_requested))
            //
            // main menu -> map transition
            .add_exit_system(GameState::Pause, ldtk::cleanup.run_if(cleanup_requested))
            //
            // menu transitions
            .add_enter_system(GameState::MainMenu, main_menu::spawn_menu)
            .add_exit_system(GameState::MainMenu, main_menu::despawn_menu)
            .add_enter_system(GameState::Pause, pause_menu::spawn_menu)
            .add_exit_system(GameState::Pause, pause_menu::despawn_menu)
            //
            // pause <-> ingame transition
            .add_system(handle_to_pause_input.run_in_state(GameState::InGame))
            .add_system(handle_to_ingame_input.run_in_state(GameState::Pause));
    }
}

const CLEAR_COLOR: Color = Color::BLACK;

#[derive(SystemLabel)]
#[allow(dead_code)]
enum GameSystem {
    Input,
    Movement,
    Combat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    Pause,
}

// events
pub struct CleanupMapEvent;
pub struct SetupMapEvent;

#[derive(Resource, Default)]
pub struct MapLevel {
    entity: Option<Entity>,
}

fn setup_requested(ev_setup: EventReader<SetupMapEvent>) -> bool {
    let res = !ev_setup.is_empty();
    ev_setup.clear();
    res
}

fn cleanup_requested(ev_cleanup: EventReader<CleanupMapEvent>) -> bool {
    let res = !ev_cleanup.is_empty();
    ev_cleanup.clear();
    res
}

fn handle_to_pause_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut ev_freeze: EventWriter<FreezePhysicsEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        pause_game(&mut commands, &mut ev_freeze);
    }
}

fn handle_to_ingame_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut ev_unfreeze: EventWriter<UnfreezePhysicsEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        resume_game(&mut commands, &mut ev_unfreeze);
    }
}

pub fn pause_game(commands: &mut Commands, ev_freeze: &mut EventWriter<FreezePhysicsEvent>) {
    commands.insert_resource(NextState(GameState::Pause));
    ev_freeze.send(FreezePhysicsEvent);
}

pub fn resume_game(commands: &mut Commands, ev_unfreeze: &mut EventWriter<UnfreezePhysicsEvent>) {
    commands.insert_resource(NextState(GameState::InGame));
    ev_unfreeze.send(UnfreezePhysicsEvent);
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
