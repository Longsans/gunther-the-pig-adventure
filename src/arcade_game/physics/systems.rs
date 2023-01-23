use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

pub fn freeze_physics(
    mut moveables: Query<(
        &mut Moveable,
        Option<&mut Velocity>,
        Option<&mut GravityScale>,
    )>,
    ev_freeze_reader: EventReader<FreezePhysicsEvent>,
) {
    if !ev_freeze_reader.is_empty() {
        ev_freeze_reader.clear();
        for (mut moveable, velocity, g_scale) in &mut moveables {
            if let Some(mut velocity) = velocity {
                moveable.freeze_velocity(&mut velocity);
            }
            if let Some(mut g_scale) = g_scale {
                moveable.freeze_gravity(&mut g_scale);
            }
        }
    }
}

pub fn unfreeze_physics(
    mut moveables: Query<(
        &mut Moveable,
        Option<&mut Velocity>,
        Option<&mut GravityScale>,
    )>,
    ev_unfreeze_reader: EventReader<UnfreezePhysicsEvent>,
) {
    if !ev_unfreeze_reader.is_empty() {
        ev_unfreeze_reader.clear();
        for (mut moveable, velocity, g_scale) in &mut moveables {
            if let Some(mut g_scale) = g_scale {
                moveable.unfreeze_gravity(&mut g_scale);
            }
            if let Some(mut velocity) = velocity {
                moveable.unfreeze_velocity(&mut velocity);
            }
        }
    }
}

pub fn freeze_requested(ev_freeze_reader: EventReader<FreezePhysicsEvent>) -> bool {
    if !ev_freeze_reader.is_empty() {
        ev_freeze_reader.clear();
        return true;
    } else {
        false
    }
}

pub fn unfreeze_requested(ev_unfreeze_reader: EventReader<UnfreezePhysicsEvent>) -> bool {
    if !ev_unfreeze_reader.is_empty() {
        ev_unfreeze_reader.clear();
        return true;
    } else {
        false
    }
}
