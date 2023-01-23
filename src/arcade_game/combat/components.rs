use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::arcade_game::physics::prelude::*;

#[derive(SystemLabel)]
pub enum CombatSystem {
    Aim,
    // Charge,
    Engage,
    Result,
}

#[derive(Resource, Default)]
pub struct SpriteSheetHandle(pub(crate) Handle<TextureAtlas>);

#[derive(Resource)]
pub struct ProjectileSpriteIndex(pub(crate) usize);

#[derive(Component, Default)]
pub struct HitPoint(pub u8);

#[derive(Component, Default)]
pub struct Damage(pub u8);

#[derive(Component, Default)]
pub struct Projectile {
    pub fx_radius: u8,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub sprite: SpriteSheetBundle,
    pub projectile: Projectile,
    pub collider: DynamicColliderBundle,
    pub moveable: Moveable,
    pub velocity: Velocity,
}

impl Projectile {
    pub const DEFAULT_SPEED: f32 = 100.0;
}
