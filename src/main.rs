pub mod asteroids;
pub mod camera;
pub mod physics;
pub mod player;
pub mod window_bounds;
pub mod window_wrap;

mod app;

fn main() {
    app::start_app()
}
