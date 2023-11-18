use bevy::{prelude::*, window::close_on_esc};

use crate::{asteroids, camera, physics, player, window_wrap};

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Startup
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                player::spawn_player,
                asteroids::spawn_asteroids,
            ),
        )
        // Input
        .add_systems(Update, (close_on_esc, player::handle_player_input))
        // Physics
        .add_systems(
            Update,
            (
                physics::translate_moveable_objects,
                physics::rotate_rotating_objects,
                window_wrap::wrap_objects_around_screen,
            ),
        )
        .run();
}
