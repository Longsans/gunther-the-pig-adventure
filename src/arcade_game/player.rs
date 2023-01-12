use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::sprite_sheets::{
    ObjectsSpriteSheet, OBJ_SPRITE_SHEET_PATH, OBJ_TILES, OBJ_TILE_OFFSET, OBJ_TILE_PADDING,
    OBJ_TILE_SIZE,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectsSpriteSheet::default())
            .insert_resource(PlayerTileIndex::PIG)
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(prep_resources.label(PlayerSystem::Resources))
                    .with_system(
                        spawn_player
                            .label(PlayerSystem::Player)
                            .after(PlayerSystem::Resources),
                    ),
            );
    }
}

#[derive(SystemLabel)]
enum PlayerSystem {
    Player,
    Resources,
}

#[derive(Resource)]
struct PlayerTileIndex(usize);

impl PlayerTileIndex {
    const PIG: Self = PlayerTileIndex(25);
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteSheetBundle,
    hp: HitPoint,
    dmg: Damage,
    name: Name,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteSheetBundle {
                transform: Self::DEFAULT_TRANSFORM,
                ..default()
            },
            hp: HitPoint(100),
            dmg: Damage(10),
            name: Name::from(Self::DEFAULT_NAME),
            player: Player,
        }
    }
}

impl PlayerBundle {
    const DEFAULT_NAME: &str = "Player";
    const DEFAULT_SCALE: f32 = 2.5;

    const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;
}

fn prep_resources(
    mut sprite_sheet: ResMut<ObjectsSpriteSheet>,
    asset_server: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tex_handle = asset_server.load(OBJ_SPRITE_SHEET_PATH);
    let atlas = TextureAtlas::from_grid(
        tex_handle,
        OBJ_TILE_SIZE,
        OBJ_TILES.rows,
        OBJ_TILES.cols,
        Some(OBJ_TILE_PADDING),
        Some(OBJ_TILE_OFFSET),
    );
    sprite_sheet.obj_tiles = tex_atlases.add(atlas);
}

fn spawn_player(
    mut commands: Commands,
    sprite_sheet: Res<ObjectsSpriteSheet>,
    tile_index: Res<PlayerTileIndex>,
) {
    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: tile_index.0,
                custom_size: Some(PlayerBundle::DEFAULT_SCALE * OBJ_TILE_SIZE),
                ..default()
            },
            texture_atlas: sprite_sheet.obj_tiles.clone(),
            transform: PlayerBundle::DEFAULT_TRANSFORM,
            ..default()
        },
        ..default()
    });
}
