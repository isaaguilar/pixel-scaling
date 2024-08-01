use crate::setup::WIDTH;
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                projection: OrthographicProjection {
                    near: -1000.0,
                    far: 1000.0,
                    scaling_mode: ScalingMode::FixedHorizontal(WIDTH),
                    ..default()
                },
                camera_2d: Camera2d { ..default() },
                camera: Camera {
                    order: 0,
                    ..default()
                },
                ..default()
            },
            RenderLayers::from_layers(&[0, 1]),
        ))
        .insert(Transform::from_xyz(0., 0., 0.));
}
