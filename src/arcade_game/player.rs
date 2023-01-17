use crate::arcade_game::character::*;
use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::physics::{Moveable, PhysicsSystem};
use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system_set(
                SystemSet::new().with_system(
                    handle_input
                        .label(GameSystem::Input)
                        .label(PhysicsSystem::Local),
                ),
            );
    }
}

#[derive(Component, Default)]
struct Player;

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
    pub const DEFAULT_NAME: &str = "Player";
    pub const DEFAULT_SCALE: f32 = 1.;
    pub const DEFAULT_MOVE_SPEED: f32 = 50.0;
    pub const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;
    pub const JUMP_FORCE: f32 = 50.0;
}

fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    mut player: Query<
        (
            &Moveable,
            &mut Character,
            &mut Transform,
            &mut Velocity,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if player.is_empty() {
        return;
    }
    let (moveable, character, mut transform, mut velocity, mut sprite) = player.single_mut();
    // velocity.linvel.x = 0.0;
    let mut movement = 0.0;
    if kb_input.pressed(KeyCode::A) || kb_input.pressed(KeyCode::Left) {
        // velocity.linvel.x -= moveable.speed;
        movement -= moveable.speed * 0.005;
    }
    if kb_input.pressed(KeyCode::D) || kb_input.pressed(KeyCode::Right) {
        // velocity.linvel.x += moveable.speed;
        movement += moveable.speed * 0.005;
    }
    if transform.translation.x != 0.0 {
        turn_player_direction(&mut sprite, movement * Vec2::X);
        // turn_player_direction(&mut sprite, velocity.linvel);
        transform.translation.x += movement;
    }

    if kb_input.just_pressed(KeyCode::Space) && character.grounded {
        // pop off the ground by an unnoticeable amount so that ground detection won't immediately ground this character
        transform.translation.y += 0.5;
        velocity.linvel.y = PlayerBundle::JUMP_FORCE;
    }
}

fn turn_player_direction(sprite: &mut TextureAtlasSprite, heading_toward: Vec2) {
    sprite.flip_x = heading_toward.x > 0.;
}
