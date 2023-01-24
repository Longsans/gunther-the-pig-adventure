use bevy::{app::AppExit, prelude::*};
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
            .add_event::<CleanupSceneEvent>()
            .add_event::<SetupSceneEvent>()
            .add_startup_system(spawn_camera)
            //
            // map -> main menu transition
            .add_enter_system_set(
                GameState::InGame,
                ConditionSet::new()
                    .run_if(setup_requested)
                    .with_system(ldtk::setup)
                    .with_system(physics::unpause_physics)
                    .into(),
            )
            //
            // main menu -> map transition
            .add_exit_system_set(
                GameState::Pause,
                ConditionSet::new()
                    .run_if(cleanup_requested)
                    .with_system(ldtk::cleanup)
                    .with_system(combat::cleanup_projectiles)
                    .into(),
            )
            //
            // menu transitions
            .add_enter_system(GameState::MainMenu, main_menu::spawn_menu)
            .add_exit_system(GameState::MainMenu, main_menu::despawn_menu)
            .add_enter_system(GameState::Pause, pause_menu::spawn_menu)
            .add_exit_system(GameState::Pause, pause_menu::despawn_menu)
            //
            // pause <-> ingame transition
            .add_system(handle_pause_game_input.run_in_state(GameState::InGame))
            .add_system(handle_resume_game_input.run_in_state(GameState::Pause));
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
pub struct CleanupSceneEvent;
pub struct SetupSceneEvent;

#[derive(Resource, Default)]
pub struct MapLevel {
    entity: Option<Entity>,
}

fn setup_requested(ev_setup: EventReader<SetupSceneEvent>) -> bool {
    let res = !ev_setup.is_empty();
    ev_setup.clear();
    res
}

fn cleanup_requested(ev_cleanup: EventReader<CleanupSceneEvent>) -> bool {
    let res = !ev_cleanup.is_empty();
    ev_cleanup.clear();
    res
}

fn handle_pause_game_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    ev_freeze: EventWriter<FreezePhysicsEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        pause_game(&mut commands, ev_freeze);
    }
}

fn handle_resume_game_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    ev_unfreeze: EventWriter<UnfreezePhysicsEvent>,
) {
    if input.just_pressed(KeyCode::Escape) {
        resume_game(&mut commands, ev_unfreeze);
    }
}

pub fn start_game(mut commands: Commands, mut ev_writer_setup: EventWriter<SetupSceneEvent>) {
    commands.insert_resource(NextState(GameState::InGame));
    ev_writer_setup.send(SetupSceneEvent);
}

pub fn pause_game(commands: &mut Commands, ev_freeze: EventWriter<FreezePhysicsEvent>) {
    commands.insert_resource(NextState(GameState::Pause));
    physics::pause_physics(ev_freeze);
}

pub fn resume_game(commands: &mut Commands, ev_unfreeze: EventWriter<UnfreezePhysicsEvent>) {
    commands.insert_resource(NextState(GameState::InGame));
    physics::unpause_physics(ev_unfreeze);
}

pub fn quit_to_main_menu(
    mut commands: Commands,
    mut ev_writer_cleanup: EventWriter<CleanupSceneEvent>,
) {
    commands.insert_resource(NextState(GameState::MainMenu));
    ev_writer_cleanup.send(CleanupSceneEvent);
}

pub fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
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
