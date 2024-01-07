// Export a bevy scene to glTF

use bevy::prelude::*;
use bevy_gltf_kun::{Export, ExportResult, GltfKunPlugin};

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
        .add_systems(Update, (export_scene, read_export_result))
        .run();
}

#[derive(Component)]
pub struct SceneMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 7.0, 3.0),
        ..default()
    });

    commands.spawn((
        SceneMarker,
        SceneBundle {
            scene: asset_server.load("BoxTextured.glb#Scene0"),
            ..default()
        },
    ));
}

fn export_scene(
    scenes: Query<Entity, With<SceneMarker>>,
    mut writer: EventWriter<Export>,
    mut exported: Local<bool>,
) {
    if *exported {
        return;
    }

    for scene in scenes.iter() {
        info!("Exporting scene...");

        writer.send(Export {
            scenes: vec![scene],
            default_scene: Some(scene),
        });

        *exported = true;
    }
}

fn read_export_result(mut reader: EventReader<ExportResult>) {
    for result in reader.read() {
        if let Ok(_glb) = &result.result {
            info!("Exported glTF!");
        }
    }
}
