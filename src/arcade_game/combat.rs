use crate::arcade_game::player::*;
use crate::arcade_game::GameSystem;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::physics::DynamicColliderBundle;
use super::physics::Moveable;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectileSpriteIndex(DEFAULT_PROJECTILE_INDEX))
            .insert_resource(SpriteSheetHandle::default())
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .label(GameSystem::Input)
                    .label(GameSystem::Combat)
                    .after(GameSystem::Movement)
                    .with_system(aim_weapon.label(CombatSystem::Aim))
                    .with_system(
                        fire_projectile
                            .label(CombatSystem::Engage)
                            .after(CombatSystem::Aim),
                    ),
            );
    }
}

const OBJECT_SPRITE_SHEET_PATH: &str = "obj_tiles.png";
const DEFAULT_PROJECTILE_INDEX: usize = 249;

#[derive(SystemLabel)]
enum CombatSystem {
    Aim,
    // Charge,
    Engage,
}

#[derive(Resource, Default)]
pub struct SpriteSheetHandle(pub(crate) Handle<TextureAtlas>);

#[derive(Resource)]
pub struct ProjectileSpriteIndex(pub(crate) usize);

#[derive(Component, Default)]
pub struct HitPoint(pub u8);

#[derive(Component, Default)]
pub struct Damage(pub u8);

#[derive(Component, Default)]
pub struct Projectile {
    pub fx_radius: u8,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub sprite: SpriteSheetBundle,
    pub projectile: Projectile,
    pub collider: DynamicColliderBundle,
    pub moveable: Moveable,
    pub velocity: Velocity,
}

impl Projectile {
    const DEFAULT_SPEED: f32 = 100.0;
}

fn setup(
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas_handle: ResMut<SpriteSheetHandle>,
) {
    let texture_handle = asset_server.load(OBJECT_SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        16.0 * Vec2::ONE,
        21,
        21,
        Some(8.0 * Vec2::ONE),
        Some(8.0 * Vec2::ONE),
    );
    *atlas_handle = SpriteSheetHandle(atlases.add(texture_atlas));
}

fn aim_weapon(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&Velocity, &mut Player), Without<PlayerChild>>,
    mut child_sprites: Query<&mut Transform, With<PlayerChild>>,
) {
    if player.is_empty() {
        return;
    }
    let (velocity, mut player) = player.single_mut();
    for mut child_transform in &mut child_sprites {
        let curr_z = child_transform.rotation.to_euler(EulerRot::YXZ).2;
        if velocity.linvel.x != 0.0 {
            let moved_right = if velocity.linvel.x > 0.0 { 1.0 } else { 0.0 };
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
        let rot_delta = f32::clamp(
            rot_delta,
            -curr_z - f32::to_radians(PlayerBundle::WEAPON_TRAJEC_MAX_ROT),
            -f32::to_radians(PlayerBundle::WEAPON_TRAJEC_MIN_ROT) - curr_z,
        );
        player.projectile_angle += rot_delta;
        child_transform.rotate_local_z(rot_delta);
    }
}

fn fire_projectile(
    kb_input: Res<Input<KeyCode>>,
    proj_sprite_sheet: Res<SpriteSheetHandle>,
    proj_sprite_index: Res<ProjectileSpriteIndex>,
    player: Query<(&GlobalTransform, &Player)>,
    mut commands: Commands,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        let (player_transform, player) = player.single();
        let projectile_angle = dir_to_sign(&player.forward) * player.projectile_angle;
        commands.spawn(ProjectileBundle {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(proj_sprite_index.0),
                texture_atlas: proj_sprite_sheet.0.clone(),
                transform: Transform::from_translation(
                    player_transform.translation() + 8.0 * player.forward.extend(0.0),
                )
                .with_scale(0.5 * Vec3::ONE),
                ..default()
            },
            collider: DynamicColliderBundle {
                collider: Collider::cuboid(4.0, 4.0),
                rigid_body: RigidBody::Dynamic,
                locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
                ..default()
            },
            moveable: Moveable {
                speed: Projectile::DEFAULT_SPEED,
                ..default()
            },
            projectile: Projectile { fx_radius: 1 },
            velocity: Velocity::linear(
                Projectile::DEFAULT_SPEED
                    * Quat::from_rotation_z(projectile_angle)
                        .mul_vec3(player.forward.extend(0.0))
                        .truncate(),
            ),
        });
    }
}

fn dir_to_sign(dir: &Vec2) -> f32 {
    if dir.x > 0.0 {
        -1.0
    } else {
        1.0
    }
}
