use arcade_game::ArcadeGame;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

mod arcade_game;

const W_WIDTH: f32 = 1280.;
const W_HEIGHT: f32 = 720.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: W_WIDTH,
                        height: W_HEIGHT,
                        title: "Hotel California".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                })
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=error,exp_game=debug".into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_plugin(ArcadeGame)
        .run();
}
