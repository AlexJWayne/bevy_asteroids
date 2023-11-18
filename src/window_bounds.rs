use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub struct OutOfBounds {
    pub left: bool,
    pub right: bool,
    pub bottom: bool,
    pub top: bool,

    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,

    pub width: f32,
    pub height: f32,
}

pub fn out_of_bounds(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    translation: Vec3,
    // diameter: f32,
) -> OutOfBounds {
    let window = window_query.get_single().unwrap();

    // let radius = diameter / 2.0;

    let width = window.width();
    let height = window.height();

    let x_max = window.width() / 2.0;
    let y_max = window.height() / 2.0;
    let x_min = -x_max;
    let y_min = -y_max;

    OutOfBounds {
        left: translation.x < x_min,
        right: translation.x > x_max,
        bottom: translation.y < y_min,
        top: translation.y > y_max,

        width,
        height,
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

pub fn randomly_positioned_translation_in_window(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    diameter: f32,
) -> Vec3 {
    let window = window_query.get_single().unwrap();
    let radius = diameter / 2.0;
    Vec3::new(
        radius + random::<f32>() * (window.width() - diameter),
        radius + random::<f32>() * (window.height() - diameter),
        0.0,
    )
}

pub fn window_center(window_query: &Query<&Window, With<PrimaryWindow>>) -> Vec3 {
    let window = window_query.get_single().unwrap();
    Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)
}
