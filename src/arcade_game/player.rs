use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::physics::Moveable;
use crate::arcade_game::sprite_sheets::{
    ObjectsSpriteSheet, OBJ_SPRITE_SHEET_PATH, OBJ_TILES, OBJ_TILE_OFFSET, OBJ_TILE_PADDING,
    OBJ_TILE_SIZE,
};
use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::arcade_game::physics::{Character, CharacterBundle, PhysicsSystem};

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
            )
            .add_system_set(
                SystemSet::new().with_system(
                    handle_input
                        .label(GameSystem::Input)
                        .label(PhysicsSystem::Local),
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

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteSheetBundle,
    hp: HitPoint,
    dmg: Damage,
    name: Name,
    player: Player,
    character_bundle: CharacterBundle,
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
            player: Player,
            character_bundle: CharacterBundle {
                moveable: Moveable {
                    speed: PlayerBundle::DEFAULT_MOVE_SPEED,
                },
                ..default()
            },
        }
    }
}

impl PlayerBundle {
    const DEFAULT_NAME: &str = "Player";
    const DEFAULT_SCALE: f32 = 2.;
    const DEFAULT_MOVE_SPEED: f32 = 1.;
    const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;
    const JUMP_FORCE: f32 = 50.0;

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
    let half_extent = PlayerBundle::default_size() / 2.;
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: tile_index.0,
                custom_size: Some(PlayerBundle::DEFAULT_SCALE * OBJ_TILE_SIZE),
                ..default()
            },
            texture_atlas: sprite_sheet.obj_tiles.clone(),
            transform: PlayerBundle::DEFAULT_TRANSFORM,
            ..default()
        },
        character_bundle: CharacterBundle {
            collider: Collider::cuboid(half_extent.x, half_extent.y),
            ..PlayerBundle::default().character_bundle
        },
        ..default()
    });
}

fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    mut q_moveable: Query<
        (
            &Moveable,
            &mut Character,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if q_moveable.is_empty() {
        return;
    }
    let (moveable, mut char, mut controller, controller_output, mut sprite) =
        q_moveable.single_mut();

    let grounded = match controller_output {
        Some(output) => output.grounded,
        None => false,
    };
    let mut direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::A) || kb_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }
    if kb_input.pressed(KeyCode::D) || kb_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }
    if direction != Vec2::ZERO {
        turn_player_direction(&mut sprite, direction);
        controller.translation = match controller.translation {
            Some(translation) => Some(translation + moveable.speed * direction),
            None => Some(moveable.speed * direction),
        }
    }
    if kb_input.just_pressed(KeyCode::Space) && grounded {
        char.jump(&mut controller, PlayerBundle::JUMP_FORCE);
    }
}

fn turn_player_direction(sprite: &mut TextureAtlasSprite, heading_toward: Vec2) {
    sprite.flip_x = heading_toward.x > 0.;
}
