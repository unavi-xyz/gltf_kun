use std::path::Path;

use bevy::{core::FrameCount, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_gltf_kun::{
    export::gltf::{GltfExport, GltfExportResult},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::{extensions::DefaultExtensions, io::format::glb::GlbIO};

const ASSETS_DIR: &str = "../assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const MODELS: &[&str] = &["BoxTextured.glb"];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: ASSETS_DIR.to_string(),
                ..default()
            }),
            EguiPlugin,
            GltfKunPlugin::<DefaultExtensions>::default(),
            PanOrbitCameraPlugin,
        ))
        .init_resource::<ExportedPath>()
        .init_resource::<SelectedModel>()
        .add_event::<LoadScene>()
        .add_systems(Startup, setup)
        .add_systems(Update, (ui, spawn_model, export, reload, get_result))
        .run();
}

#[derive(Component)]
struct SceneMarker;

#[derive(Event)]
struct LoadScene(String);

#[derive(Default, Resource)]
struct SelectedModel(String);

#[derive(Default, Resource)]
struct ExportedPath(String);

fn setup(mut commands: Commands, mut writer: EventWriter<LoadScene>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 5.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 7.0, 3.0),
        ..default()
    });

    writer.send(LoadScene(MODELS[0].to_string()));
}

fn ui(
    mut contexts: EguiContexts,
    mut writer: EventWriter<LoadScene>,
    mut selected: ResMut<SelectedModel>,
    mut exported: ResMut<ExportedPath>,
) {
    if selected.0.is_empty() {
        selected.0 = MODELS[0].to_string();
    }

    bevy_egui::egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Click and drag to orbit camera");
        ui.label("Scroll to zoom camera");
        ui.label("Press 'e' to test export");
        ui.label("Press 'r' to re-load the scene");

        ui.separator();

        bevy_egui::egui::ComboBox::from_label("Model")
            .selected_text(selected.0.as_str())
            .show_ui(ui, |ui| {
                for model in MODELS {
                    if ui
                        .selectable_label(selected.0.as_str() == *model, *model)
                        .clicked()
                    {
                        selected.0 = model.to_string();
                        exported.0.clear();
                        writer.send(LoadScene(model.to_string()));
                    }
                }
            });
    });
}

fn spawn_model(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut events: EventReader<LoadScene>,
    scenes: Query<Entity, With<SceneMarker>>,
) {
    for event in events.read() {
        for entity in scenes.iter() {
            commands.entity(entity).despawn_recursive();
        }

        info!("Spawning model {}", event.0);

        commands.spawn((
            SceneBundle {
                scene: asset_server.load(format!("{}#Scene0", event.0)),
                ..default()
            },
            SceneMarker,
        ));
    }
}

fn export(
    mut export: EventWriter<GltfExport<DefaultExtensions>>,
    mut key_events: EventReader<ReceivedCharacter>,
    scene: Query<&Handle<Scene>, With<SceneMarker>>,
) {
    for event in key_events.read() {
        if !event.char.eq_ignore_ascii_case("e") {
            continue;
        }

        info!("Exporting scene");

        let handle = match scene.get_single() {
            Ok(handle) => handle,
            Err(e) => {
                error!("Failed to get scene: {}", e);
                return;
            }
        };

        export.send(GltfExport::new(handle.clone()));
    }
}

fn reload(
    mut writer: EventWriter<LoadScene>,
    mut key_events: EventReader<ReceivedCharacter>,
    exported: Res<ExportedPath>,
    selected: Res<SelectedModel>,
) {
    for event in key_events.read() {
        if !event.char.eq_ignore_ascii_case("r") {
            continue;
        }

        let mut used_path = exported.0.clone();

        if used_path.is_empty() {
            used_path = selected.0.clone();
        }

        info!("Reloading scene");

        writer.send(LoadScene(used_path));
    }
}

fn get_result(
    mut exports: ResMut<Events<GltfExportResult>>,
    mut writer: EventWriter<LoadScene>,
    frame: Res<FrameCount>,
    mut exported_path: ResMut<ExportedPath>,
) {
    for mut event in exports.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export from Bevy: {}", e),
        };

        let glb = match GlbIO::<DefaultExtensions>::export(&mut event.graph, &doc) {
            Ok(glb) => glb,
            Err(e) => panic!("Failed to export to glb: {}", e),
        };

        let temp_dir = Path::new(CARGO_MANIFEST_DIR)
            .join(ASSETS_DIR)
            .join(TEMP_FOLDER);

        // Delete and re-create temp dir
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone()).expect("Failed to delete temp directory");
        }

        std::fs::create_dir_all(temp_dir).expect("Failed to create temp directory");

        // Write glb to temp dir
        let file_path = Path::new(TEMP_FOLDER).join(temp_file(frame.0));
        let file_path_str = file_path.to_str().unwrap().to_string();
        exported_path.0 = file_path_str.clone();

        info!("Writing glb to {}", file_path.display());

        let full_path = Path::new(CARGO_MANIFEST_DIR)
            .join(ASSETS_DIR)
            .join(file_path);

        std::fs::write(full_path, glb.0).expect("Failed to write glb");

        // Load glb
        writer.send(LoadScene(file_path_str));
    }
}

const TEMP_FOLDER: &str = "temp/bevy_gltf";

fn temp_file(frame: u32) -> String {
    format!("model_{}.glb", frame)
}
