use bevy::prelude::*;
use bevy_gltf_kun::GltfKunPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            }),
            GltfKunPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 7.0, 3.0),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("BoxTextured.glb#Scene0"),
        ..default()
    });
}
