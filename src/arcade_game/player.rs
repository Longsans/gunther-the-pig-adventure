use crate::arcade_game::character::*;
use crate::arcade_game::combat::{Damage, HitPoint};
use crate::arcade_game::physics::{self, Moveable, PhysicsSystem};
use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_weapon_trajectory).add_system_set(
            SystemSet::new()
                .with_system(update_grounded_status.before(GameSystem::Input))
                .with_system(
                    handle_input
                        .label(GameSystem::Input)
                        .label(PhysicsSystem::Local),
                ),
        );
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default)]
struct PlayerChild;

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
                    moved: false,
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
    pub const WEAPON_TRAJEC_LEN: f32 = 30.0;
    pub const WEAPON_TRAJEC_ROT: f32 = 30.0;
    pub const WEAPON_TRAJEC_MAX_ROT: f32 = 60.0;
    pub const WEAPON_TRAJEC_MIN_ROT: f32 = -60.0;
}

fn add_weapon_trajectory(
    player: Query<Entity, Added<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for entity in &player {
        commands.entity(entity).with_children(|child_builder| {
            child_builder
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(-0.2 * Vec3::Z),
                        ..default()
                    },
                    PlayerChild,
                ))
                .with_children(|inner_child_builder| {
                    inner_child_builder.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Box::new(PlayerBundle::WEAPON_TRAJEC_LEN, 0.2, 0.01).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::RED)).into(),
                        transform: Transform::from_translation(
                            -PlayerBundle::WEAPON_TRAJEC_LEN / 2.0 * Vec3::X,
                        ),
                        ..default()
                    });
                });
        });
    }
}

fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<
        (
            &mut Moveable,
            &mut Character,
            &mut Transform,
            &mut Velocity,
            &mut TextureAtlasSprite,
        ),
        (With<Player>, Without<PlayerChild>),
    >,
    mut child_sprites: Query<&mut Transform, With<PlayerChild>>,
) {
    if player.is_empty() {
        return;
    }
    let (moveable, character, mut transform, mut velocity, mut sprite) = player.single_mut();

    velocity.linvel.x = 0.0;
    if kb_input.pressed(KeyCode::A) || kb_input.pressed(KeyCode::Left) {
        velocity.linvel.x -= moveable.speed;
    }
    if kb_input.pressed(KeyCode::D) || kb_input.pressed(KeyCode::Right) {
        velocity.linvel.x += moveable.speed;
    }
    for mut child_transform in &mut child_sprites {
        let curr_z = child_transform.rotation.to_euler(EulerRot::YXZ).2;
        if velocity.linvel.x != 0.0 {
            let moved_right = velocity.linvel.x > 0.0;
            sprite.flip_x = moved_right;
            let moved_right = if moved_right { 1.0 } else { 0.0 };

            child_transform.rotation = Quat::from_euler(
                EulerRot::YXZ,
                moved_right * std::f32::consts::PI,
                0.0,
                curr_z,
            );
        }
        let mut rot_delta = 0.0;
        if kb_input.pressed(KeyCode::W) || kb_input.pressed(KeyCode::Up) {
            rot_delta = time.delta_seconds() * f32::to_radians(-PlayerBundle::WEAPON_TRAJEC_ROT);
        }
        if kb_input.pressed(KeyCode::S) || kb_input.pressed(KeyCode::Down) {
            rot_delta = time.delta_seconds() * f32::to_radians(PlayerBundle::WEAPON_TRAJEC_ROT);
        }
        child_transform.rotate_local_z(f32::clamp(
            rot_delta,
            PlayerBundle::WEAPON_TRAJEC_MIN_ROT + curr_z,
            PlayerBundle::WEAPON_TRAJEC_MAX_ROT - curr_z,
        ));
    }

    if kb_input.just_pressed(KeyCode::Space) && character.grounded {
        // pop off the ground by an unnoticeable amount so that ground detection won't immediately ground this character
        transform.translation.y += 0.5;
        velocity.linvel.y = PlayerBundle::JUMP_FORCE;
    }
}

fn update_grounded_status(
    mut velocities: Query<(&GlobalTransform, &Collider, Option<&mut Character>)>,
    rapier_context: Res<RapierContext>,
) {
    for (g_transform, collider, character) in &mut velocities {
        if physics::detect_grounded(&rapier_context, g_transform, collider) {
            if let Some(mut character) = character {
                character.grounded = true;
            }
            continue;
        } else {
            if let Some(mut character) = character {
                character.grounded = false;
            }
        }
    }
}
