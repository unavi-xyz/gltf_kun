// Round-trip example of loading a glTF into Bevy, exporting it, and loading it again.
// This is useful for ensuring that both the importer and exporter are working correctly.

use bevy::prelude::*;
use bevy_gltf_kun::{
    export::{Export, ExportResult},
    GltfKunPlugin,
};
use gltf_kun::{
    document::GltfDocument,
    io::format::{gltf::GltfFormat, ExportFormat},
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
        .init_resource::<ExportTimer>()
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

#[derive(Resource)]
struct ExportTimer(Timer);

impl Default for ExportTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

fn export_scene(
    scenes: Query<Entity, With<Handle<Scene>>>,
    mut writer: EventWriter<Export<GltfDocument>>,
    mut timer: ResMut<ExportTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    for scene in scenes.iter() {
        info!("Exporting scene...");

        writer.send(Export {
            scenes: vec![scene],
            default_scene: Some(scene),
            ..default()
        });
    }
}

fn read_export_result(
    _commands: Commands,
    mut events: ResMut<Events<ExportResult<GltfDocument>>>,
    _scenes: Query<Entity, With<Handle<Scene>>>,
    _asset_server: Res<AssetServer>,
) {
    for event in events.drain() {
        if let Ok(doc) = event.result {
            let gltf = GltfFormat::export(doc).expect("Failed export to GLB");
            let json = serde_json::to_string_pretty(&gltf.json).expect("Failed to serialize JSON");
            info!("Got exported gltf:\n{}", json);

            // // Now clear the scene and load the exported GLB.
            // for scene in scenes.iter() {
            //     commands.entity(scene).despawn_recursive();
            // }
            //
            // info!("Loading exported GLB...");
            // commands.spawn(SceneBundle {
            //     scene: asset_server.load(format!("{}#Scene0", path)),
            //     ..default()
            // });
        }
    }
}
