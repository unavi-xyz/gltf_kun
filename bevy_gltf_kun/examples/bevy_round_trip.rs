// Round-trip example of loading a glTF into Bevy, exporting it, and loading it again.
// This is useful for ensuring that both the importer and exporter are working correctly.

use std::path::Path;

use bevy::prelude::*;
use bevy_gltf_kun::{
    export::{Export, ExportResult},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::{
    document::GltfDocument,
    io::format::{gltf::GltfFormat, ExportFormat},
};

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const EXPORTED_MODEL: &str = "temp/bevy_round_trip/model.gltf";
const MODEL: &str = "BoxTextured.glb";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: ASSETS_DIR.to_string(),
                ..default()
            }),
            GltfKunPlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (export_scene, read_export_result))
        .init_resource::<ExportTimer>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 7.0, 3.0),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load(format!("{}#Scene0", MODEL)),
        ..default()
    });
}

#[derive(Resource)]
struct ExportTimer(Timer);

impl Default for ExportTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
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
    mut commands: Commands,
    mut events: ResMut<Events<ExportResult<GltfDocument>>>,
    scenes: Query<Entity, With<Handle<Scene>>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.drain() {
        if let Ok(doc) = event.result {
            let gltf = GltfFormat::export(doc).expect("Failed export glTF");

            let json = serde_json::to_string_pretty(&gltf.json).expect("Failed to serialize JSON");
            info!("Got exported glTF:\n{}", json);

            // Write to file
            let assets = Path::new(CARGO_MANIFEST_DIR).join(ASSETS_DIR);
            let path = assets.join(EXPORTED_MODEL);
            std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
            gltf.write_file(&assets.join(path))
                .expect("Failed to write glTF to file");

            // Now clear the scene and load the exported GLB.
            info!("Clearing scene...");
            for scene in scenes.iter() {
                commands.entity(scene).despawn_recursive();
            }

            info!("Loading exported model...");
            commands.spawn(SceneBundle {
                scene: asset_server.load(format!("{}#Scene0", EXPORTED_MODEL)),
                ..default()
            });
        }
    }
}
