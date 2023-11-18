use bevy::{prelude::*, window::PrimaryWindow};

use crate::window_bounds::out_of_bounds;

pub fn wrap_objects_around_screen(
    mut moveable_objects_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in moveable_objects_query.iter_mut() {
        let out_of_bounds = out_of_bounds(&window_query, transform.translation);
        if out_of_bounds.left {
            transform.translation.x += out_of_bounds.width;
        }
        if out_of_bounds.right {
            transform.translation.x -= out_of_bounds.width;
        }
        if out_of_bounds.bottom {
            transform.translation.y += out_of_bounds.height;
        }
        if out_of_bounds.top {
            transform.translation.y -= out_of_bounds.height;
        }
    }
}
