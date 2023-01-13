use crate::arcade_game::GameSystem;
use bevy::prelude::*;

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_collider.before(GameSystem::Physics))
            .register_type::<Collider>();
    }
}

#[derive(Component, Default)]
pub struct Moveable {
    pub speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Collider {
    pub col_type: ColliderType,
    pub min: Vec2,
    pub max: Vec2,
    pub extent: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            col_type: ColliderType::Static,
            min: Vec2::ZERO,
            max: Vec2::ZERO,
            extent: Vec2::ZERO,
        }
    }
}

impl Collider {
    #[allow(dead_code)]
    pub fn collide(&self, other: &Collider) -> bool {
        let (top_left, top_right, bot_right, bot_left) = self.collide_corners(&other);
        top_left || top_right || bot_right || bot_left
    }

    pub fn collide_corners(&self, other: &Collider) -> (bool, bool, bool, bool) {
        let top_left_collide = self.min.x.in_range_phys(other.min.x, other.max.x)
            && self.max.y.in_range_phys(other.min.y, other.max.y);
        let top_right_collide = self.max.x.in_range_phys(other.min.x, other.max.x)
            && self.max.y.in_range_phys(other.min.y, other.max.y);
        let bot_left_collide = self.min.x.in_range_phys(other.min.x, other.max.x)
            && self.min.y.in_range_phys(other.min.y, other.max.y);
        let bot_right_collide = self.min.y.in_range_phys(other.min.y, other.max.y)
            && self.max.x.in_range_phys(other.min.x, other.max.x);
        (
            top_left_collide,
            top_right_collide,
            bot_right_collide,
            bot_left_collide,
        )
    }
}

#[derive(Reflect)]
pub enum ColliderType {
    Static,
    // Trigger,
}

trait InRangePhysics {
    fn in_range_phys(self, low: Self, high: Self) -> bool;
}

impl InRangePhysics for f32 {
    fn in_range_phys(self, low: Self, high: Self) -> bool {
        low - self <= 5. && self - high <= 5.
    }
}

fn update_collider(mut q: Query<(&GlobalTransform, &mut Collider)>) {
    for (gl_transform, mut collider) in &mut q {
        let pos = Vec2::new(gl_transform.translation().x, gl_transform.translation().y);
        let min = pos - collider.extent;
        let max = pos + collider.extent;
        if min != collider.min {
            collider.min = min;
            collider.max = max;
        }
    }
}
