//! glTF importing and exporting.

use std::path::Path;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
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
        .add_event::<LoadScene>()
        .add_systems(Startup, setup)
        .add_systems(Update, (ui, spawn_model, export, get_result))
        .run();
}

#[derive(Component)]
struct SceneMarker;

fn setup(mut commands: Commands, mut writer: EventWriter<LoadScene>) {
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

    writer.send(LoadScene(MODELS[0].to_string()));
}

#[derive(Event)]
struct LoadScene(String);

fn ui(mut contexts: EguiContexts, mut writer: EventWriter<LoadScene>, mut selected: Local<String>) {
    if selected.is_empty() {
        *selected = MODELS[0].to_string();
    }

    bevy_egui::egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Click and drag to orbit camera");
        ui.label("Scroll to zoom camera");
        ui.label("Press 'e' to test export");

        ui.separator();

        bevy_egui::egui::ComboBox::from_label("Model")
            .selected_text(selected.as_str())
            .show_ui(ui, |ui| {
                for model in MODELS {
                    if ui
                        .selectable_label(selected.as_str() == *model, *model)
                        .clicked()
                    {
                        *selected = model.to_string();
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
    mut keyboard_input: EventReader<KeyboardInput>,
    mut last_export: Local<f32>,
    scene: Query<&Handle<Scene>, With<SceneMarker>>,
    time: Res<Time>,
) {
    if !keyboard_input
        .read()
        .any(|e| e.key_code == Some(KeyCode::E))
    {
        return;
    }

    if time.elapsed_seconds() - *last_export < 0.5 {
        return;
    }

    *last_export = time.elapsed_seconds();

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

const TEMP_FILE: &str = "temp/bevy_gltf/model.glb";

fn get_result(mut exports: ResMut<Events<GltfExportResult>>, mut writer: EventWriter<LoadScene>) {
    for mut event in exports.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export from Bevy: {}", e),
        };

        let glb = match GlbIO::<DefaultExtensions>::export(&mut event.graph, &doc) {
            Ok(glb) => glb,
            Err(e) => panic!("Failed to export to glb: {}", e),
        };

        let path = Path::new(CARGO_MANIFEST_DIR)
            .join(ASSETS_DIR)
            .join(TEMP_FILE);

        info!("Writing glb to {}", path.display());

        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create temp directory");
        std::fs::write(path, glb.0).expect("Failed to write glb");

        writer.send(LoadScene(TEMP_FILE.to_string()));
    }
}
