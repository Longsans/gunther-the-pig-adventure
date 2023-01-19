use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::arcade_game::physics::StaticColliderBundle;

pub const TILE_SIZE: Vec2 = Vec2 { x: 8., y: 8. };

#[derive(Component, Default)]
pub struct Terrain;

impl Terrain {
    const DEFAULT_NAME: &str = "Terrain";
    const UPHILL_NAME: &str = "Uphill";
    const DOWNHILL_NAME: &str = "Downhill";
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

#[derive(Bundle, LdtkIntCell)]
pub struct UphillTerrainBundle {
    #[ldtk_int_cell]
    #[bundle]
    uphill_collider_bundle: StaticColliderBundle,
    terrain: Terrain,
    name: Name,
}

#[derive(Bundle, LdtkIntCell)]
pub struct DownhillTerrainBundle {
    #[ldtk_int_cell]
    #[bundle]
    downhill_collider_bundle: StaticColliderBundle,
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

impl Default for UphillTerrainBundle {
    fn default() -> Self {
        Self {
            name: Name::from(Terrain::UPHILL_NAME),
            ..default()
        }
    }
}

impl Default for DownhillTerrainBundle {
    fn default() -> Self {
        Self {
            name: Name::from(Terrain::DOWNHILL_NAME),
            ..default()
        }
    }
}
