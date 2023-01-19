use crate::arcade_game::physics::{DynamicColliderBundle, Moveable};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectileSpriteIndex(249))
            .insert_resource(SpriteSheetHandle::default())
            .add_startup_system(setup);
    }
}

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

#[derive(Resource, Default)]
pub struct SpriteSheetHandle(pub(crate) Handle<TextureAtlas>);

#[derive(Resource)]
pub struct ProjectileSpriteIndex(pub(crate) usize);

const OBJECT_SPRITE_SHEET_PATH: &str = "obj_tiles.png";

fn setup(
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas_handle: ResMut<SpriteSheetHandle>,
) {
    let texture_handle = asset_server.load(OBJECT_SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        16.0 * Vec2::ONE,
        21,
        21,
        Some(8.0 * Vec2::ONE),
        Some(8.0 * Vec2::ONE),
    );
    *atlas_handle = SpriteSheetHandle(atlases.add(texture_atlas));
}
