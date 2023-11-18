use std::f32::consts::PI;

use crate::{
    physics::{AngularMomentum, Velocity},
    window_bounds::randomly_positioned_translation_in_window,
};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const ASTEROID_MIN_SPEED: f32 = 30.0;
const ASTEROID_MAX_SPEED: f32 = 120.0;
const ASTEROID_MAX_ROTATION_SPEED: f32 = 0.3 * (PI * 2.0);

pub fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..10 {
        let asteroid_sprite_path = format!(
            "sprites/Meteors/spaceMeteors_00{}.png",
            rand::thread_rng().gen_range(1..=4)
        );
        commands.spawn((
            Velocity::from_random_direction(ASTEROID_MIN_SPEED, ASTEROID_MAX_SPEED),
            AngularMomentum((random::<f32>() * 2.0 - 1.0) * ASTEROID_MAX_ROTATION_SPEED),
            SpriteBundle {
                transform: Transform {
                    translation: randomly_positioned_translation_in_window(&window_query, 0.0),
                    scale: Vec3::splat(0.15),
                    ..default()
                },
                texture: asset_server.load(asteroid_sprite_path),
                ..default()
            },
        ));
    }
}
