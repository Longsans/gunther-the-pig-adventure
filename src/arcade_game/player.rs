use crate::arcade_game::GameSystem;
use bevy::prelude::*;

pub mod components;
pub mod prelude;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::add_weapon_trajectory)
            .add_system_set(
                SystemSet::new().with_system(
                    systems::handle_input
                        .label(GameSystem::Input)
                        .label(GameSystem::Movement),
                ),
            );
    }
}
