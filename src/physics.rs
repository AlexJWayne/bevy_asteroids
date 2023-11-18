use std::f32::consts::PI;

use bevy::prelude::*;
use rand::random;

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn from_random_direction(min_speed: f32, max_speed: f32) -> Velocity {
        let direction = Vec2::from_angle(random::<f32>() * PI * 2.0);
        let speed = min_speed + random::<f32>() * (max_speed - min_speed);
        Velocity(direction * speed)
    }
}

#[derive(Component)]
pub struct AngularMomentum(pub f32);

#[derive(Component)]
pub struct Diameter(pub f32);

pub fn translate_moveable_objects(
    mut moveable_objects_query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in moveable_objects_query.iter_mut() {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

pub fn rotate_rotating_objects(
    mut rotating_objects_query: Query<(&mut Transform, &AngularMomentum)>,
    time: Res<Time>,
) {
    for (mut transform, angular_momentum) in rotating_objects_query.iter_mut() {
        transform.rotate_z(angular_momentum.0 * time.delta_seconds());
    }
}
