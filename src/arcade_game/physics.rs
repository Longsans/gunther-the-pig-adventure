use crate::arcade_game::character;
use crate::arcade_game::ldtk;
use crate::arcade_game::map;
use crate::arcade_game::player::PlayerBundle;
use bevy::prelude::*;
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
        app.add_system(
            simulate_gravity
                .label(PhysicsSystem::GlobalSimulation)
                .after(PhysicsSystem::Local),
        );
    }
}

const GRAVITY: f32 = 9.8;
const GRAVITY_SCALE: f32 = 32.0;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Moveable {
    pub speed: f32,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct DynamicColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct StaticColliderBundle {
    pub collider: Collider,
    pub locked_axes: LockedAxes,
}

impl From<IntGridCell> for StaticColliderBundle {
    fn from(cell: IntGridCell) -> Self {
        let terrain_half_extents = map::TILE_SIZE / 2.0;
        match cell.value {
            ldtk::TERRAIN | ldtk::PLATFORM => Self {
                collider: Collider::cuboid(terrain_half_extents.x, terrain_half_extents.y),
                locked_axes: LockedAxes::ROTATION_LOCKED,
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
            },
            _ => Self::default(),
        }
    }
}

fn simulate_gravity(mut velocities: Query<&mut Velocity>, time: Res<Time>) {
    for mut velocity in &mut velocities {
        velocity.linvel.y -= time.delta_seconds() * GRAVITY_SCALE * GRAVITY;
    }
}
