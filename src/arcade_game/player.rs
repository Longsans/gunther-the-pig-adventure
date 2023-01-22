use crate::arcade_game::{GameState, GameSystem};
use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

pub mod components;
pub mod prelude;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::add_weapon_trajectory)
            .add_system_set(
                ConditionSet::new()
                    .label(GameSystem::Input)
                    .label(GameSystem::Movement)
                    .run_in_state(GameState::InGame)
                    .with_system(systems::handle_input)
                    .into(),
            );
    }
}
