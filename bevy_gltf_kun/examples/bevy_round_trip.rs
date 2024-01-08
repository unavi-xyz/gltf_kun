// Round-trip example of loading a glTF into Bevy, exporting it, and loading it again.
// This is useful for ensuring that both the importer and exporter are working correctly.

use bevy::prelude::*;
use bevy_gltf_kun::{
    export::{Export, ExportResult},
    GltfKunPlugin,
};
use gltf_kun::{
    document::GltfDocument,
    io::format::{glb::GlbFormat, ExportFormat},
};

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

fn export_scene(
    scenes: Query<Entity, With<Handle<Scene>>>,
    mut writer: EventWriter<Export<GltfDocument>>,
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
            ..default()
        });

        *exported = true;
    }
}

fn read_export_result(
    mut commands: Commands,
    mut events: ResMut<Events<ExportResult<GltfDocument>>>,
    scenes: Query<Entity, With<Handle<Scene>>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.drain() {
        if let Ok(doc) = event.result {
            let glb = GlbFormat::export(doc).expect("Failed export to GLB");
            info!("Got exported GLB! Size: {}", format_byte_length(&glb.0));

            let dir = std::path::Path::new("assets/temp");
            std::fs::create_dir_all(dir).expect("Failed to create temp directory");
            let path = dir.join("round_trip.glb");
            info!("Writing GLB to {:?}", path);
            std::fs::write(path, glb.0).expect("Failed to write GLB");

            // Now clear the scene and load the exported GLB.
            for scene in scenes.iter() {
                commands.entity(scene).despawn_recursive();
            }

            info!("Loading exported GLB...");
            commands.spawn(SceneBundle {
                scene: asset_server.load("temp/round_trip.glb#Scene0"),
                ..default()
            });
        }
    }
}

fn format_byte_length(bytes: &[u8]) -> String {
    let len = bytes.len() as f32;
    let mut unit = "B";

    if len > 1024.0 {
        unit = "KB";
    }

    if len > 1024.0 * 1024.0 {
        unit = "MB";
    }

    if len > 1024.0 * 1024.0 * 1024.0 {
        unit = "GB";
    }

    format!("{:.2} {}", len, unit)
}
