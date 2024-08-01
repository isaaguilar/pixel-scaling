use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
    window::{PresentMode, WindowResolution},
};

pub const WIDTH: f32 = 640.;

pub struct WindowSetup;

impl Plugin for WindowSetup {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WIDTH, 480.),
                        present_mode: PresentMode::AutoNoVsync,
                        title: "Resizing".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=info".to_string(),
                    custom_layer: |_| None,
                }),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgb(0.623, 0.689, 0.876)));
    }
}
