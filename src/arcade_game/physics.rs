use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

pub mod components;
pub mod events;
pub mod prelude;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            })
            .add_event::<components::FreezePhysicsEvent>()
            .add_event::<components::UnfreezePhysicsEvent>()
            .add_system(systems::freeze_physics.run_if(systems::freeze_requested))
            .add_system(systems::unfreeze_physics.run_if(systems::unfreeze_requested));
    }
}
