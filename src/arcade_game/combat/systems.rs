use crate::arcade_game::map::*;
use crate::arcade_game::player::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand;
use rand::Rng;

use super::components::*;
use crate::arcade_game::physics::DynamicColliderBundle;
use crate::arcade_game::physics::Moveable;

fn dir_to_sign(dir: &Vec2) -> f32 {
    if dir.x > 0.0 {
        -1.0
    } else {
        1.0
    }
}

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut atlas_handle: ResMut<SpriteSheetHandle>,
) {
    let texture_handle = asset_server.load(super::OBJECT_SPRITE_SHEET_PATH);
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

pub fn aim_weapon(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&KinematicCharacterController, &mut Player), Without<PlayerChild>>,
    mut child_sprites: Query<&mut Transform, With<PlayerChild>>,
) {
    if player.is_empty() {
        return;
    }
    let (controller, mut player) = player.single_mut();
    for mut child_transform in &mut child_sprites {
        let curr_z = child_transform.rotation.to_euler(EulerRot::YXZ).2;
        // handle aim sprite flip on movement
        if let Some(translation) = controller.translation {
            if translation.x != 0.0 {
                let moved_right = if translation.x > 0.0 { 1.0 } else { 0.0 };
                child_transform.rotation = Quat::from_euler(
                    EulerRot::YXZ,
                    moved_right * std::f32::consts::PI,
                    0.0,
                    curr_z,
                );
            }
        }
        // handle input
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

pub fn fire_projectile(
    kb_input: Res<Input<KeyCode>>,
    proj_sprite_sheet: Res<SpriteSheetHandle>,
    proj_sprite_index: Res<ProjectileSpriteIndex>,
    player: Query<(&GlobalTransform, &Player)>,
    mut commands: Commands,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        let (player_transform, player) = player.single();
        let projectile_angle = dir_to_sign(&player.forward) * player.projectile_angle;
        commands
            .spawn(ProjectileBundle {
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(proj_sprite_index.0),
                    texture_atlas: proj_sprite_sheet.0.clone(),
                    transform: Transform::from_translation(
                        player_transform.translation() + 4.0 * player.forward.extend(0.0),
                    )
                    .with_scale(0.5 * Vec3::ONE),
                    ..default()
                },
                collider: DynamicColliderBundle {
                    collider: Collider::cuboid(4.0, 4.0),
                    rigid_body: RigidBody::Dynamic,
                    locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
                    collision_groups: DynamicColliderBundle::proj_collision_groups(),
                    ..default()
                },
                moveable: Moveable {
                    speed: Projectile::DEFAULT_SPEED,
                    ..default()
                },
                projectile: Projectile { fx_radius: 1 },
                velocity: Velocity {
                    linvel: Projectile::DEFAULT_SPEED
                        * Quat::from_rotation_z(projectile_angle)
                            .mul_vec3(player.forward.extend(0.0))
                            .truncate(),
                    angvel: rand::thread_rng().gen_range(-30.0..30.0),
                },
            })
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

pub fn deal_projectile_effect(
    terrain_tiles: Query<(Entity, &GlobalTransform, &Collider), With<Terrain>>,
    projectiles: Query<(Entity, &Projectile)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for collision in collision_events.iter() {
        if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
            if let Ok((_, tile_transform, tile_collider)) = terrain_tiles.get(*entity_one) {
                if let Ok((proj_entity, projectile)) = projectiles.get(*entity_two) {
                    despawn_nearby_tiles(
                        tile_transform,
                        tile_collider,
                        projectile,
                        &terrain_tiles,
                        &mut commands,
                    );
                    commands.entity(proj_entity).despawn_recursive();
                }
            } else if let Ok((_, tile_transform, tile_collider)) = terrain_tiles.get(*entity_two) {
                if let Ok((proj_entity, projectile)) = projectiles.get(*entity_one) {
                    despawn_nearby_tiles(
                        tile_transform,
                        tile_collider,
                        projectile,
                        &terrain_tiles,
                        &mut commands,
                    );
                    commands.entity(proj_entity).despawn_recursive();
                }
            }
        }
    }
}

fn despawn_nearby_tiles(
    tile_transform: &GlobalTransform,
    tile_collider: &Collider,
    projectile: &Projectile,
    terrain_tiles: &Query<(Entity, &GlobalTransform, &Collider), With<Terrain>>,
    commands: &mut Commands,
) {
    if let Some(collider) = tile_collider.as_cuboid() {
        let fx_area_x = (
            tile_transform.translation().x
                - projectile.fx_radius as f32 * collider.half_extents().x,
            tile_transform.translation().x
                + projectile.fx_radius as f32 * collider.half_extents().x,
        );
        let fx_area_y = (
            tile_transform.translation().y
                - projectile.fx_radius as f32 * collider.half_extents().y,
            tile_transform.translation().y
                + projectile.fx_radius as f32 * collider.half_extents().y,
        );
        let adjacent_tiles = terrain_tiles.iter().filter(|(_, g_transform, _)| {
            let translation_x = g_transform.translation().x;
            let translation_y = g_transform.translation().y;
            translation_x >= fx_area_x.0
                && translation_x <= fx_area_x.1
                && translation_y >= fx_area_y.0
                && translation_y <= fx_area_y.1
        });
        for (entity, _, _) in adjacent_tiles {
            commands.entity(entity).despawn_recursive();
        }
    } else {
        let entity = terrain_tiles
            .iter()
            .find(|(_, g_transform, _)| g_transform.translation() == tile_transform.translation());
        if let Some((entity, _, _)) = entity {
            commands.entity(entity).despawn_recursive();
        }
    }
}
