use crate::arcade_game::character::*;
use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::physics::Moveable;
use crate::arcade_game::physics::PhysicsSystem;
use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
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
    pub const DEFAULT_MOVE_SPEED: f32 = 0.5;
    pub const DEFAULT_TRANSFORM: Transform = Transform::IDENTITY;
    pub const JUMP_FORCE: f32 = 100.0;
}

fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    mut q_moveable: Query<
        (
            &Moveable,
            &mut Velocity,
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
    let (moveable, mut velocity, mut controller, controller_output, mut sprite) =
        q_moveable.single_mut();

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
        };
    }

    let grounded = match controller_output {
        Some(output) => output.grounded,
        None => false,
    };
    if kb_input.just_pressed(KeyCode::Space) && grounded {
        velocity.linvel.y = PlayerBundle::JUMP_FORCE;
    }
}

fn turn_player_direction(sprite: &mut TextureAtlasSprite, heading_toward: Vec2) {
    sprite.flip_x = heading_toward.x > 0.;
}
