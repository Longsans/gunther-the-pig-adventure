use crate::arcade_game::character;
use crate::arcade_game::ldtk;
use crate::arcade_game::map;
use crate::arcade_game::player::prelude::PlayerBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(dead_code)]
const GRAVITY: f32 = 9.8;
const GRAVITY_SCALE: f32 = 1.0;

pub struct FreezePhysicsEvent;
pub struct UnfreezePhysicsEvent;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Moveable {
    pub speed: f32,
    frozen_speed: f32,
    frozen_linvel: Vec2,
    frozen_angvel: f32,
    frozen_g_scale: f32,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct DynamicColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub gravity_scale: GravityScale,
    pub collision_groups: CollisionGroups,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct StaticColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub locked_axes: LockedAxes,
    pub collision_groups: CollisionGroups,
}

impl Moveable {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            frozen_speed: 0.0,
            frozen_linvel: Vec2::ZERO,
            frozen_angvel: 0.0,
            frozen_g_scale: 1.0,
        }
    }
}

impl StaticColliderBundle {
    pub fn collision_groups() -> CollisionGroups {
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_3)
    }
}

impl DynamicColliderBundle {
    pub fn player_collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::GROUP_1,
            filters: Group::GROUP_2,
        }
    }

    pub fn proj_collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::GROUP_3,
            filters: Group::GROUP_2 | Group::GROUP_3,
        }
    }
}

impl From<IntGridCell> for StaticColliderBundle {
    fn from(cell: IntGridCell) -> Self {
        let terrain_half_extents = map::TILE_SIZE / 2.0;
        let def_fric = Friction {
            coefficient: 0.0,
            ..default()
        };
        let collision_groups = StaticColliderBundle::collision_groups();
        match cell.value {
            ldtk::TERRAIN | ldtk::PLATFORM | ldtk::PLATFORM_PATTERN => Self {
                collider: Collider::cuboid(terrain_half_extents.x, terrain_half_extents.y),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
                collision_groups,
            },
            ldtk::UPHILL_TERRAIN => Self {
                collider: Collider::triangle(
                    map::TILE_SIZE * 0.5 * Vec2::NEG_ONE,
                    map::TILE_SIZE * 0.5 * Vec2::ONE,
                    map::TILE_SIZE * 0.5 * (Vec2::X - Vec2::Y),
                ),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
                collision_groups,
            },
            ldtk::DOWNHILL_TERRAIN => Self {
                collider: Collider::triangle(
                    map::TILE_SIZE * 0.5 * Vec2::NEG_ONE,
                    map::TILE_SIZE * 0.5 * (Vec2::X - Vec2::Y),
                    map::TILE_SIZE * 0.5 * (-Vec2::X + Vec2::Y),
                ),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
                collision_groups,
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
                gravity_scale: GravityScale(GRAVITY_SCALE),
                collision_groups: DynamicColliderBundle::player_collision_groups(),
                ..default()
            },
            _ => Self::default(),
        }
    }
}
