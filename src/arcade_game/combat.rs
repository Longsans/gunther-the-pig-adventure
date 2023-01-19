use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use components::*;
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
            .add_system_set(
                SystemSet::new()
                    .label(GameSystem::Input)
                    .label(GameSystem::Combat)
                    .after(GameSystem::Movement)
                    .with_system(aim_weapon.label(CombatSystem::Aim))
                    .with_system(
                        fire_projectile
                            .label(CombatSystem::Engage)
                            .after(CombatSystem::Aim),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .label(GameSystem::Combat)
                    .label(CombatSystem::Result)
                    .after(CombatSystem::Engage)
                    .with_system(deal_projectile_effect),
            );
    }
}

const OBJECT_SPRITE_SHEET_PATH: &str = "obj_tiles.png";
const DEFAULT_PROJECTILE_INDEX: usize = 203;
