use crate::arcade_game::character::{self, Character};
use crate::arcade_game::ldtk;
use crate::arcade_game::map;
use crate::arcade_game::player::PlayerBundle;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(SystemLabel)]
pub enum PhysicsSystem {
    Local,
    GlobalSimulation,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            simulate_gravity
                .label(PhysicsSystem::GlobalSimulation)
                .after(PhysicsSystem::Local)
                .after(TransformSystem::TransformPropagate),
        );
    }
}

const GRAVITY: f32 = 9.8;
const GRAVITY_SCALE: f32 = 1.0;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Moveable {
    pub speed: f32,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct DynamicColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct StaticColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub locked_axes: LockedAxes,
}

impl From<IntGridCell> for StaticColliderBundle {
    fn from(cell: IntGridCell) -> Self {
        let terrain_half_extents = map::TILE_SIZE / 2.0;
        match cell.value {
            ldtk::TERRAIN | ldtk::PLATFORM | ldtk::PLATFORM_PATTERN => Self {
                collider: Collider::cuboid(terrain_half_extents.x, terrain_half_extents.y),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: Friction {
                    coefficient: 0.0,
                    ..default()
                },
            },
            _ => Self::default(),
        }
    }
}

impl From<EntityInstance> for DynamicColliderBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        let player_half_extents = PlayerBundle::DEFAULT_SCALE * character::CHARACTER_SIZE;
        match entity_instance.identifier.as_ref() {
            ldtk::PLAYER_ID => Self {
                collider: Collider::cuboid(player_half_extents.x, player_half_extents.y),
                rigid_body: RigidBody::Dynamic,
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: Friction {
                    coefficient: 0.0,
                    ..default()
                },
            },
            _ => Self::default(),
        }
    }
}

fn simulate_gravity(
    mut velocities: Query<(
        &mut Velocity,
        &GlobalTransform,
        &Collider,
        Option<&mut Character>,
    )>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    for (mut velocity, g_transform, collider, character) in &mut velocities {
        if detect_grounded(&rapier_context, g_transform, collider) {
            velocity.linvel.y = 0.0;
            if let Some(mut character) = character {
                character.grounded = true;
            }
            continue;
        } else {
            if let Some(mut character) = character {
                character.grounded = false;
            }
            velocity.linvel.y -= time.delta_seconds() * GRAVITY_SCALE * GRAVITY;
        }
    }
}

fn detect_grounded(
    rapier_context: &RapierContext,
    g_transform: &GlobalTransform,
    collider: &Collider,
) -> bool {
    if let Some(cube) = collider.as_cuboid() {
        let shape_pos =
            g_transform.translation().clone().truncate() - (cube.half_extents().y + 0.1) * Vec2::Y;
        let shape = Collider::cuboid(cube.half_extents().x - 0.5, 0.00001);
        let max_toi = 0.001;
        let shape_vel = max_toi * Vec2::Y;
        let filter = QueryFilter::default();

        if let Some(_) =
            rapier_context.cast_shape(shape_pos, 0.0, shape_vel, &shape, max_toi, filter)
        {
            return true;
        }
        return false;
    } else {
        false
    }
}
