use bevy::prelude::*;
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
        )
        .register_type::<Character>();
    }
}

#[derive(Component, Default)]
pub struct Moveable {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Character;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub moveable: Moveable,
    pub character: Character,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub gravity_scale: GravityScale,
    pub controller: KinematicCharacterController,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            moveable: Moveable { speed: 0.0 },
            character: Character::default(),
            collider: Collider::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            gravity_scale: GravityScale(1.0),
            controller: KinematicCharacterController::default(),
        }
    }
}

impl Character {
    pub fn jump(&mut self, controller: &mut KinematicCharacterController, jump_force: f32) {
        controller.translation = Some(Vec2::new(0., jump_force));
    }
}

#[allow(dead_code)]
fn check_grounded(
    rapier_context: &RapierContext,
    g_transform: &GlobalTransform,
    collider: &Collider,
) -> bool {
    if let Some(cube) = collider.as_cuboid() {
        let left_foot =
            g_transform.translation().clone().truncate() - cube.half_extents() + 0.01 * Vec2::NEG_Y;
        let right_foot = left_foot + 2.0 * cube.half_extents().x * Vec2::X + 0.01 * Vec2::NEG_Y;

        let ray_dir = Vec2::NEG_Y;
        let max_toi = 0.1;
        let solid = false;
        let filter = QueryFilter::default();
        let feet = [left_foot, right_foot];
        for foot in &feet {
            if let Some(_) = rapier_context.cast_ray(*foot, ray_dir, max_toi, solid, filter) {
                return true;
            }
        }
        return false;
    } else {
        false
    }
}

fn simulate_gravity(mut controllers: Query<&mut KinematicCharacterController>) {
    let gravity = 0.1 * Vec2::new(0., -9.8);
    for mut con in &mut controllers {
        con.translation = match con.translation {
            Some(con) => Some(con + gravity),
            None => Some(gravity),
        };
    }
}
