//! Round-trip example of importing a gltf, exporting it, then re-importing it again.
//! This is useful for debugging and testing, ensuring consistent behavior between import and export.

use std::path::Path;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_gltf_kun::{
    export::{Export, GltfExport, GltfExportResult},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::{extensions::DefaultExtensions, io::format::glb::GlbIO};

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const MODELS: &[&str] = &["BoxTextured.glb", "DynamicBox.gltf"];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: ASSETS_DIR.to_string(),
                ..default()
            }),
            EguiPlugin,
            GltfKunPlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (ui, spawn_model, export, get_result))
        .insert_resource(SelectedModel(MODELS[0].to_string()))
        .run();
}

#[derive(Component)]
struct SceneMarker;

fn setup(mut commands: Commands) {
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
}

#[derive(Resource)]
struct SelectedModel(String);

fn ui(mut contexts: EguiContexts, mut selected_model: ResMut<SelectedModel>) {
    bevy_egui::egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Click and drag to orbit camera");
        ui.label("Scroll to zoom camera");
        ui.label("Press 'e' to test export");

        ui.separator();

        ui.label("Model");
        ui.horizontal(|ui| {
            for model in MODELS {
                if ui.button(*model).clicked() {
                    selected_model.0 = model.to_string();
                }
            }
        });
    });
}

fn spawn_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_model: Res<SelectedModel>,
    mut current_model: Local<String>,
    scenes: Query<Entity, With<SceneMarker>>,
) {
    if selected_model.0 == *current_model {
        return;
    }

    for entity in scenes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    info!("Spawning model {}", selected_model.0);

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(format!("{}#Scene0", selected_model.0)),
            ..default()
        },
        SceneMarker,
    ));

    *current_model = selected_model.0.clone();
}

fn export(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut export: EventWriter<GltfExport>,
    scene: Query<&Handle<Scene>, With<SceneMarker>>,
) {
    if !keyboard_input
        .read()
        .any(|e| e.key_code == Some(KeyCode::E))
    {
        return;
    }

    info!("Exporting scene");

    let handle = match scene.get_single() {
        Ok(handle) => handle,
        Err(e) => {
            error!("Failed to get scene: {}", e);
            return;
        }
    };

    export.send(Export::new(handle.clone()));
}

const TEMP_FILE: &str = "temp/bevy_round_trip/model.glb";

fn get_result(
    mut events: ResMut<Events<GltfExportResult>>,
    mut selected_model: ResMut<SelectedModel>,
) {
    for mut event in events.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export from Bevy: {}", e),
        };

        let io = GlbIO;
        let glb = match io.export(&mut event.graph, &doc, Some(&DefaultExtensions)) {
            Ok(glb) => glb,
            Err(e) => panic!("Failed to export to glb: {}", e),
        };

        let path = Path::new(CARGO_MANIFEST_DIR)
            .join(ASSETS_DIR)
            .join(TEMP_FILE);

        info!("Writing glb to {}", path.display());

        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create temp directory");
        std::fs::write(path, glb.0).expect("Failed to write glb");

        selected_model.0 = TEMP_FILE.to_string();
    }
}
