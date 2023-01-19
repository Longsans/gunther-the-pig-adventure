use crate::arcade_game::physics::Moveable;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use super::components::*;

pub fn add_weapon_trajectory(
    player: Query<Entity, Added<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for entity in &player {
        commands.entity(entity).with_children(|child_builder| {
            child_builder
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(-0.2 * Vec3::Z),
                        ..default()
                    },
                    PlayerChild,
                ))
                .with_children(|inner_child_builder| {
                    inner_child_builder.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Box::new(PlayerBundle::WEAPON_TRAJEC_LEN, 0.2, 0.01).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::RED)).into(),
                        transform: Transform::from_translation(
                            -PlayerBundle::WEAPON_TRAJEC_LEN / 2.0 * Vec3::X,
                        ),
                        ..default()
                    });
                });
        });
    }
}

pub fn handle_input(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(
        &mut Moveable,
        &mut Player,
        &mut KinematicCharacterController,
        &mut TextureAtlasSprite,
    )>,
) {
    if player.is_empty() {
        return;
    }
    let (moveable, mut player, mut controller, mut sprite) = player.single_mut();

    let mut direction = Vec2::ZERO;
    if kb_input.pressed(KeyCode::A) || kb_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::D) || kb_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if direction != Vec2::ZERO {
        sprite.flip_x = direction.x > 0.0;
        player.forward = direction;
        direction *= time.delta_seconds();
        controller.translation = match controller.translation {
            Some(translation) => Some(translation + moveable.speed * direction),
            None => Some(moveable.speed * direction),
        };
    }
}
