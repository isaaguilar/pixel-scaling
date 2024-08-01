use bevy::prelude::{App, States};
use bevy::state::app::AppExtStates as _;

mod camera;
mod player;
mod setup;

pub const WORLD_SCALE: f32 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    Default,
}

fn main() {
    App::new()
        .add_plugins((
            setup::WindowSetup,
            camera::CameraPlugin,
            player::PlayerPlugin,
        ))
        .init_state::<AppState>()
        .run();
}
