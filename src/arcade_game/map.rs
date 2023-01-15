use crate::arcade_game::sprite_sheets::{
    MapSpriteSheet, MAP_SPRITE_SHEET_PATH, MAP_TILES, MAP_TILE_SIZE,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapSpriteSheet::default())
            .insert_resource(MapConfig {
                wall_scale: 1.,
                wall_tile_index: 48,
                wall_width: 600.,
                wall_height: 360.,
            })
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(prep_resources.label("resources"))
                    .with_system(draw_map.after("resources")),
            );
    }
}

#[derive(Resource)]
struct MapConfig {
    wall_scale: f32,
    wall_tile_index: usize,
    wall_width: f32,
    wall_height: f32,
}

impl MapConfig {
    fn tile_size(&self) -> Vec2 {
        self.wall_scale * MAP_TILE_SIZE
    }
}

#[derive(Component)]
pub struct Wall;

impl Wall {
    const DEFAULT_NAME: &str = "Wall";
}

#[derive(Bundle)]
struct WallBundle {
    collider: Collider,
    sprite: SpriteSheetBundle,
    wall: Wall,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            collider: Collider::default(),
            sprite: SpriteSheetBundle::default(),
            wall: Wall,
        }
    }
}

fn prep_resources(
    mut sprite_sheet: ResMut<MapSpriteSheet>,
    asset_server: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tex_handle = asset_server.load(MAP_SPRITE_SHEET_PATH);
    let atlas = TextureAtlas::from_grid(
        tex_handle,
        MAP_TILE_SIZE,
        MAP_TILES.rows,
        MAP_TILES.cols,
        None,
        None,
    );
    sprite_sheet.map_tiles = tex_atlases.add(atlas);
}

fn draw_map(mut commands: Commands, sprite_sheet: Res<MapSpriteSheet>, map_config: Res<MapConfig>) {
    let tile_width = map_config.tile_size().x;
    let tile_height = map_config.tile_size().y;
    let mut start_pos = Transform::from_xyz(
        map_config.wall_width / 2. - tile_width / 2.,
        map_config.wall_height / 2. - tile_height / 2.,
        0.,
    )
    .translation;
    let h_walls = (map_config.wall_width / tile_width) as i32;
    let v_walls = (map_config.wall_height / tile_height) as i32;
    info!("h_walls: {h_walls}");
    info!("v_walls: {v_walls}");

    let wall_parent = commands
        .spawn(SpatialBundle {
            visibility: Visibility::VISIBLE,
            ..default()
        })
        .insert(Name::from("Walls"))
        .id();
    for _ in 1..h_walls {
        let up_wall = spawn_tiles(
            &mut commands,
            &sprite_sheet,
            &map_config,
            Transform::from_translation(start_pos),
        );
        let down_wall = spawn_tiles(
            &mut commands,
            &sprite_sheet,
            &map_config,
            Transform::from_translation(
                start_pos - (map_config.wall_height - tile_height) * Vec3::Y,
            ),
        );
        commands
            .entity(wall_parent)
            .push_children(&[up_wall, down_wall]);
        start_pos.x -= tile_width;
    }

    for _ in 0..v_walls {
        let left_wall = spawn_tiles(
            &mut commands,
            &sprite_sheet,
            &map_config,
            Transform::from_translation(start_pos),
        );
        let right_wall = spawn_tiles(
            &mut commands,
            &sprite_sheet,
            &map_config,
            Transform::from_translation(start_pos + (map_config.wall_width - tile_width) * Vec3::X),
        );
        commands
            .entity(wall_parent)
            .push_children(&[left_wall, right_wall]);
        start_pos.y -= tile_height;
    }
}

fn spawn_tiles(
    commands: &mut Commands,
    sprite_sheet: &MapSpriteSheet,
    map_config: &MapConfig,
    transform: Transform,
) -> Entity {
    let half_extent = map_config.tile_size() / 2.0;
    commands
        .spawn(WallBundle {
            collider: Collider::cuboid(half_extent.x, half_extent.y),
            sprite: SpriteSheetBundle {
                texture_atlas: sprite_sheet.map_tiles.clone(),
                sprite: TextureAtlasSprite {
                    index: map_config.wall_tile_index,
                    custom_size: Some(map_config.tile_size()),
                    ..default()
                },
                transform,
                ..default()
            },
            ..default()
        })
        .insert(Name::from(Wall::DEFAULT_NAME))
        .id()
}
