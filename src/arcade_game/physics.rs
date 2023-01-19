use crate::arcade_game::character;
use crate::arcade_game::ldtk;
use crate::arcade_game::map;
use crate::arcade_game::player::PlayerBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            });
    }
}

#[allow(dead_code)]
const GRAVITY: f32 = 9.8;
const GRAVITY_SCALE: f32 = 1.0;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Moveable {
    pub speed: f32,
    pub moved: bool,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct DynamicColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub gravity_scale: GravityScale,
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
        let def_fric = Friction {
            coefficient: 0.0,
            ..default()
        };
        match cell.value {
            ldtk::TERRAIN | ldtk::PLATFORM | ldtk::PLATFORM_PATTERN => Self {
                collider: Collider::cuboid(terrain_half_extents.x, terrain_half_extents.y),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
            },
            ldtk::UPHILL_TERRAIN => Self {
                collider: Collider::triangle(
                    map::TILE_SIZE * 0.5 * Vec2::NEG_ONE,
                    map::TILE_SIZE * 0.5 * Vec2::ONE,
                    map::TILE_SIZE * 0.5 * (Vec2::X - Vec2::Y),
                ),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
            },
            ldtk::DOWNHILL_TERRAIN => Self {
                collider: Collider::triangle(
                    map::TILE_SIZE * 0.5 * Vec2::NEG_ONE,
                    map::TILE_SIZE * 0.5 * (Vec2::X - Vec2::Y),
                    map::TILE_SIZE * 0.5 * (-Vec2::X + Vec2::Y),
                ),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: def_fric,
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
            },
            _ => Self::default(),
        }
    }
}

pub fn detect_grounded(
    rapier_context: &RapierContext,
    g_transform: &GlobalTransform,
    collider: &Collider,
) -> bool {
    if let Some(cube) = collider.as_cuboid() {
        let shape_pos =
            g_transform.translation().clone().truncate() - (cube.half_extents().y + 0.1) * Vec2::Y;
        let shape = Collider::cuboid(cube.half_extents().x, 0.01);
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
        return false;
    }
}
