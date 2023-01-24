use crate::arcade_game::{GameState, GameSystem};
use bevy::prelude::*;
use components::*;
use iyes_loopless::prelude::*;
pub use systems::cleanup_projectiles;
use systems::*;

pub mod components;
pub mod prelude;
mod systems;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectileSpriteIndex(DEFAULT_PROJECTILE_INDEX))
            .insert_resource(SpriteSheetHandle::default())
            .add_startup_system(setup)
            .add_system(
                aim_weapon
                    .run_in_state(GameState::InGame)
                    .label(GameSystem::Input)
                    .label(GameSystem::Combat)
                    .label(CombatSystem::Aim)
                    .after(GameSystem::Movement),
            )
            .add_system(
                fire_projectile
                    .run_in_state(GameState::InGame)
                    .label(GameSystem::Input)
                    .label(GameSystem::Combat)
                    .label(CombatSystem::Engage)
                    .after(CombatSystem::Aim),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .label(GameSystem::Combat)
                    .label(CombatSystem::Result)
                    .after(CombatSystem::Engage)
                    .with_system(deal_projectile_effect)
                    .into(),
            );
    }
}

const OBJECT_SPRITE_SHEET_PATH: &str = "obj_tiles.png";
const DEFAULT_PROJECTILE_INDEX: usize = 203;
