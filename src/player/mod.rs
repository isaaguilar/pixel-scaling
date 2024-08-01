use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("4circles.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("floor.png"),
        transform: Transform::from_xyz(0., -100., 0.),
        ..default()
    });
}
