use bevy::prelude::*;
use bevy_gltf_kun::{
    export::{Export, GltfExport, GltfExportResult},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::io::format::gltf::GltfFormat;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
            GltfKunPlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, import)
        .add_systems(Update, (export, log_result))
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
    scenes: Query<&Handle<Scene>, With<SceneMarker>>,
) {
    if time.elapsed_seconds() < 3.0 {
        return;
    }

    if *exported {
        return;
    }

    info!("Exporting scene...");

    let scene = scenes.get_single().expect("Failed to get scene handle");
    export.send(Export::new(scene.clone()));
    *exported = true;
}

// Log the result of the export
fn log_result(mut events: ResMut<Events<GltfExportResult>>) {
    for event in events.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export gltf from Bevy: {}", e),
        };

        let gltf = match GltfFormat::export(doc) {
            Ok(gltf) => gltf,
            Err(e) => panic!("Failed to export gltf document: {}", e),
        };

        let json = serde_json::to_string_pretty(&gltf.json).expect("Failed to serialize gltf");
        info!("Got exported gltf:\n{}", json);
    }
}
