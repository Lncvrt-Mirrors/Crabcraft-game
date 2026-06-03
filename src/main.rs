use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("blocks/grass.gltf#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 5.0)
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
