use crate::arcade_game::character::*;
use crate::arcade_game::combat::prelude::*;
use crate::arcade_game::physics::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Player {
    pub projectile_angle: f32,
    pub forward: Vec2,
}

#[derive(Component, Default)]
pub struct PlayerChild;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    character_bundle: CharacterBundle,
    player: Player,
    name: Name,
    hp: HitPoint,
    dmg: Damage,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            projectile_angle: 0.0,
            forward: Vec2::NEG_X,
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteSheetBundle {
                transform: Self::DEFAULT_TRANSFORM,
                ..default()
            },
            hp: HitPoint(100),
            dmg: Damage(10),
            name: Name::from(Self::DEFAULT_NAME),
            character_bundle: CharacterBundle {
                moveable: Moveable::new(PlayerBundle::DEFAULT_MOVE_SPEED),
                ..default()
            },
            ..default()
        }
    }
}

impl PlayerBundle {
    pub const DEFAULT_NAME: &str = "Player";
    pub const DEFAULT_SCALE: f32 = 1.;
    pub const DEFAULT_MOVE_SPEED: f32 = 35.0;
    pub const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;
    pub const WEAPON_TRAJEC_LEN: f32 = 30.0;
    pub const WEAPON_TRAJEC_ROT: f32 = 30.0;
    pub const WEAPON_TRAJEC_MAX_ROT: f32 = 60.0;
    pub const WEAPON_TRAJEC_MIN_ROT: f32 = -60.0;
}
