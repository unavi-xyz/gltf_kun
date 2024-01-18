//! Round-trip example of importing a gltf, exporting it, then re-importing it again.
//! This is useful for debugging and testing, ensuring consistent behavior between import and export.

use std::path::Path;

use bevy::prelude::*;
use bevy_gltf_kun::{
    export::{Export, GltfExport, GltfExportResult},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::io::format::{glb::GlbIO, DocumentIO};

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

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
        .add_systems(Startup, import)
        .add_systems(Update, (export, get_result))
        .run();
}

#[derive(Component)]
pub struct SceneMarker;

// Set up the scene and import a glb model
fn import(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("BoxTextured.glb#Scene0"),
            ..default()
        },
        SceneMarker,
    ));
}

// After a short delay, export the scene
fn export(
    time: Res<Time>,
    mut exported: Local<bool>,
    mut export: EventWriter<GltfExport>,
    scene: Query<&Handle<Scene>, With<SceneMarker>>,
) {
    if time.elapsed_seconds() < 4.0 {
        return;
    }

    if *exported {
        return;
    }

    info!("Exporting scene");

    let handle = scene.get_single().expect("Failed to get scene handle");
    export.send(Export::new(handle.clone()));
    *exported = true;
}

const TEMP_FILE: &str = "temp/bevy_round_trip/model.glb";

// Get the exported gltf, clear the scene, and re-import it
fn get_result(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut events: ResMut<Events<GltfExportResult>>,
    scenes: Query<Entity, With<SceneMarker>>,
) {
    for mut event in events.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export from Bevy: {}", e),
        };

        let io = GlbIO::default();
        let glb = match io.export(&mut event.graph, &doc) {
            Ok(glb) => glb,
            Err(e) => panic!("Failed to export to glb: {}", e),
        };

        let path = Path::new(CARGO_MANIFEST_DIR)
            .join(ASSETS_DIR)
            .join(TEMP_FILE);

        info!("Writing glb to {}", path.display());

        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create temp directory");
        std::fs::write(path, glb.0).expect("Failed to write glb");

        info!("Clearing scene");

        for entity in scenes.iter() {
            commands.entity(entity).despawn_recursive();
        }

        info!("Re-importing glb");

        commands.spawn((
            SceneBundle {
                scene: asset_server.load(format!("{}#Scene0", TEMP_FILE)),
                ..default()
            },
            SceneMarker,
        ));
    }
}
