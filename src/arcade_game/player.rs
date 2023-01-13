use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::map::Wall;
use crate::arcade_game::rigid_body::*;
use crate::arcade_game::sprite_sheets::{
    ObjectsSpriteSheet, OBJ_SPRITE_SHEET_PATH, OBJ_TILES, OBJ_TILE_OFFSET, OBJ_TILE_PADDING,
    OBJ_TILE_SIZE,
};
use crate::arcade_game::GameSystem;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectsSpriteSheet::default())
            .insert_resource(PlayerTileIndex::PIG)
            .insert_resource(AllowedMovement::default())
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(prep_resources.label(PlayerSystem::Resources))
                    .with_system(
                        spawn_player
                            .label(PlayerSystem::Player)
                            .after(PlayerSystem::Resources),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(detect_collision.label(GameSystem::Physics))
                    .with_system(
                        handle_input
                            .label(GameSystem::Input)
                            .after(GameSystem::Physics),
                    ),
            );
    }
}

#[derive(SystemLabel)]
enum PlayerSystem {
    Resources,
    Player,
}

#[derive(Resource)]
struct PlayerTileIndex(usize);

impl PlayerTileIndex {
    const PIG: Self = PlayerTileIndex(25);
}

#[derive(Resource, Debug)]
struct AllowedMovement {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Default for AllowedMovement {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
            up: true,
            down: true,
        }
    }
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
    moveable: Moveable,
    collider: Collider,
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
            moveable: Moveable {
                speed: PlayerBundle::DEFAULT_MOVE_SPEED,
            },
            collider: Collider::default(),
        }
    }
}

impl PlayerBundle {
    const DEFAULT_NAME: &str = "Player";
    const DEFAULT_SCALE: f32 = 2.;
    const DEFAULT_MOVE_SPEED: f32 = 150.;
    const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;

    fn default_size() -> Vec2 {
        Self::DEFAULT_SCALE * OBJ_TILE_SIZE
    }
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
    let default_pos = Transform::default().translation;
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
        collider: Collider {
            min: Vec2::new(default_pos.x, default_pos.y) - PlayerBundle::default_size() / 2.,
            max: Vec2::new(default_pos.x, default_pos.y) + PlayerBundle::default_size() / 2.,
            extent: PlayerBundle::default_size() / 2.,
            ..default()
        },
        ..default()
    });
}

fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    mut q_moveable: Query<(&Moveable, &mut Transform, &mut TextureAtlasSprite), With<Player>>,
    allowed_move: Res<AllowedMovement>,
    time: Res<Time>,
) {
    let (moveable, mut transform, mut sprite) = q_moveable.single_mut();
    let mut direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::W) && allowed_move.up {
        direction.y += 1.;
    }
    if kb_input.pressed(KeyCode::A) && allowed_move.left {
        direction.x -= 1.;
    }
    if kb_input.pressed(KeyCode::S) && allowed_move.down {
        direction.y -= 1.;
    }
    if kb_input.pressed(KeyCode::D) && allowed_move.right {
        direction.x += 1.;
    }
    if direction != Vec2::ZERO {
        turn_player_direction(&mut sprite, direction);
        transform.translation +=
            moveable.speed * time.delta_seconds() * Vec3::from((direction.normalize(), 0.));
    }
}

fn turn_player_direction(sprite: &mut TextureAtlasSprite, heading_toward: Vec2) {
    sprite.flip_x = heading_toward.x > 0.;
}

fn detect_collision(
    q_player: Query<&Collider, With<Player>>,
    q_walls: Query<&Collider, With<Wall>>,
    mut allowed_move: ResMut<AllowedMovement>,
) {
    let player = q_player.single();
    let mut corners_collide = (false, false, false, false);
    for walls in &q_walls {
        let corners = player.collide_corners(&walls);
        corners_collide = (
            corners_collide.0 || corners.0,
            corners_collide.1 || corners.1,
            corners_collide.2 || corners.2,
            corners_collide.3 || corners.3,
        );
    }
    allowed_move.up = !(corners_collide.0 && corners_collide.1);
    allowed_move.right = !(corners_collide.1 && corners_collide.2);
    allowed_move.down = !(corners_collide.2 && corners_collide.3);
    allowed_move.left = !(corners_collide.0 && corners_collide.3);
}
