use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use super::components::*;

pub fn freeze_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}

pub fn unfreeze_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
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
