use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::arcade_game::physics::StaticColliderBundle;

pub const TILE_SIZE: Vec2 = Vec2 { x: 8., y: 8. };

#[derive(Component, Default)]
pub struct Terrain;

impl Terrain {
    const DEFAULT_NAME: &str = "Wall";
}

#[derive(Component, Default)]
pub struct Background;

#[derive(Bundle, LdtkIntCell)]
pub struct BackgroundBundle {
    is_bg: Background,
}

#[derive(Bundle, LdtkIntCell)]
pub struct TerrainBundle {
    #[from_int_grid_cell]
    #[bundle]
    static_collider_bundle: StaticColliderBundle,
    terrain: Terrain,
    name: Name,
}

impl Default for TerrainBundle {
    fn default() -> Self {
        Self {
            static_collider_bundle: StaticColliderBundle::default(),
            terrain: Terrain,
            name: Name::from(Terrain::DEFAULT_NAME),
        }
    }
}
