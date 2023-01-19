use crate::arcade_game::map::*;
use crate::arcade_game::player::PlayerBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub const MAP_PATH: &str = "map.ldtk";

pub const PLAYER_ID: &str = "Player";

pub const TERRAIN: i32 = 1;
pub const UPHILL_TERRAIN: i32 = 8;
pub const DOWNHILL_TERRAIN: i32 = 9;
pub const FLORA: i32 = 2;
pub const TILE_ON_TOP: i32 = 3;
pub const PLANT_FOOT: i32 = 4;
pub const PLATFORM: i32 = 5;
pub const CHAIN: i32 = 6;
pub const PLATFORM_PATTERN: i32 = 7;

pub struct LdtkMapBackendPlugin;

impl Plugin for LdtkMapBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<TerrainBundle>(TERRAIN)
            .register_ldtk_int_cell::<UphillTerrainBundle>(UPHILL_TERRAIN)
            .register_ldtk_int_cell::<DownhillTerrainBundle>(DOWNHILL_TERRAIN)
            .register_ldtk_int_cell::<TerrainBundle>(PLATFORM)
            .register_ldtk_int_cell::<TerrainBundle>(PLATFORM_PATTERN)
            .register_ldtk_int_cell::<BackgroundBundle>(FLORA)
            .register_ldtk_int_cell::<BackgroundBundle>(TILE_ON_TOP)
            .register_ldtk_int_cell::<BackgroundBundle>(PLANT_FOOT)
            .register_ldtk_int_cell::<BackgroundBundle>(CHAIN)
            .register_ldtk_entity::<PlayerBundle>(PLAYER_ID)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load(MAP_PATH),
            transform: Transform::from_xyz(-128.0, -128.0, 0.0),
            ..default()
        })
        .insert(Name::from("LDtk World"));
}
