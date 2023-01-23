use crate::arcade_game::ldtk;
use crate::arcade_game::physics::prelude::*;
use crate::arcade_game::player::prelude::PlayerBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub const CHARACTER_SIZE: Vec2 = Vec2 { x: 8., y: 8. };

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Character;

#[derive(Bundle, Default, LdtkEntity)]
pub struct CharacterBundle {
    pub moveable: Moveable,
    pub character: Character,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: DynamicColliderBundle,
    pub controller: KinematicCharacterController,
    pub velocity: Velocity,
}

impl From<EntityInstance> for CharacterBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            ldtk::PLAYER_ID => Self {
                moveable: Moveable::new(PlayerBundle::DEFAULT_MOVE_SPEED),
                collider_bundle: entity_instance.into(),
                ..default()
            },
            _ => Self::default(),
        }
    }
}
