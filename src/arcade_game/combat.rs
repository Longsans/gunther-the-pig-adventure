use bevy::prelude::*;

#[derive(Component, Default)]
pub struct HitPoint(pub u8);

#[derive(Component, Default)]
pub struct Damage(pub u8);
