use bevy::prelude::*;
use std::f32::consts::PI;

use crate::physics::Velocity;

const PLAYER_ROTATION_SPEED: f32 = (2.0 * PI) * 0.5;
const PLAYER_ACCELERATION: f32 = 650.0;
const PLAYER_MAX_SPEED: f32 = 650.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ThrustPlume;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_id = commands
        .spawn((
            Player {},
            Velocity(Vec2::ZERO),
            SpriteBundle {
                transform: Transform {
                    scale: Vec3::splat(0.25),
                    ..default()
                },
                texture: asset_server.load("sprites/Ships/spaceShips_001.png"),
                ..default()
            },
        ))
        .id();

    let plume_id = commands
        .spawn((
            ThrustPlume {},
            SpriteBundle {
                texture: asset_server.load("sprites/Effects/spaceEffects_018.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 150.0, 0.0),
                    scale: Vec3::new(3.0, -3.0, 3.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands.entity(ship_id).push_children(&[plume_id]);
}

pub fn handle_player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut plume_query: Query<&mut Visibility, With<ThrustPlume>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_velocity) = player_query.get_single_mut().unwrap();
    let mut plume_visibility = plume_query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::A) {
        player_transform.rotate_z(PLAYER_ROTATION_SPEED * time.delta_seconds())
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_seconds())
    }

    if keyboard_input.pressed(KeyCode::W) {
        let movement_direction = player_transform.rotation * -Vec3::Y;
        player_velocity.0 += Vec2::new(movement_direction.x, movement_direction.y)
            * PLAYER_ACCELERATION
            * time.delta_seconds();
        *plume_visibility = Visibility::Visible;
    } else {
        *plume_visibility = Visibility::Hidden;
    }

    if player_velocity.0.length() > PLAYER_MAX_SPEED {
        player_velocity.0 = player_velocity.0.normalize() * PLAYER_MAX_SPEED;
    }
}
