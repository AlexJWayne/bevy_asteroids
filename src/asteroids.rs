use crate::{
    physics::{AngularMomentum, Velocity},
    window_bounds::randomly_positioned_translation_in_window,
};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..10 {
        commands.spawn((
            Velocity(Vec2::new(
                (random::<f32>() - 0.5) * 300.0,
                (random::<f32>() - 0.5) * 300.0,
            )),
            AngularMomentum((random::<f32>() - 0.5) * 2.0),
            SpriteBundle {
                transform: Transform {
                    translation: randomly_positioned_translation_in_window(&window_query, 0.0),
                    scale: Vec3::splat(0.15),
                    ..default()
                },
                texture: asset_server.load("sprites/Meteors/spaceMeteors_001.png"),
                ..default()
            },
        ));
    }
}
