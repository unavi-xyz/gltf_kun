use std::{fmt::Display, path::Path};

use bevy::{asset::AssetMetaCheck, core::FrameCount, gltf::Gltf, prelude::*};
use bevy_egui::{egui::ComboBox, EguiContexts, EguiPlugin};
use bevy_gltf_kun::{
    export::gltf::{GltfExport, GltfExportResult},
    import::gltf::GltfKun,
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_xpbd_3d::prelude::*;
use gltf_kun::{extensions::DefaultExtensions, io::format::glb::GlbIO};

const ASSETS_DIR: &str = "assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const MODELS: &[&str] = &[
    "BoxTextured.glb",
    "BoxTextured/BoxTextured.gltf",
    "DynamicBox.gltf",
];

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            GltfKunPlugin::<DefaultExtensions>::default(),
            PanOrbitCameraPlugin,
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .add_event::<LoadModel>()
        .add_event::<LoadScene>()
        .init_resource::<ExportedPath>()
        .init_resource::<Loader>()
        .init_resource::<SelectedModel>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                ui,
                (load_scene, load_model).chain(),
                export,
                reload,
                get_result,
            ),
        )
        .run();
}

#[derive(Event)]
struct LoadModel(String);

#[derive(Event)]
struct LoadScene(GltfHandle);

#[derive(Clone)]
enum GltfHandle {
    Bevy(Handle<Gltf>),
    GltfKun(Handle<GltfKun>),
}

impl Default for GltfHandle {
    fn default() -> Self {
        GltfHandle::GltfKun(Default::default())
    }
}

#[derive(Default, Resource)]
struct SelectedModel(String);

#[derive(Default, Resource)]
struct Loader(GltfLoader);

#[derive(Default)]
enum GltfLoader {
    BevyGltf,
    #[default]
    GltfKun,
}

impl Display for GltfLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GltfLoader::BevyGltf => write!(f, "bevy_gltf"),
            GltfLoader::GltfKun => write!(f, "gltf_kun"),
        }
    }
}

#[derive(Default, Resource)]
struct ExportedPath(String);

fn setup(mut commands: Commands, mut writer: EventWriter<LoadModel>) {
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

    writer.send(LoadModel(MODELS[0].to_string()));
}

fn ui(
    mut contexts: EguiContexts,
    mut exported: ResMut<ExportedPath>,
    mut loader: ResMut<Loader>,
    mut selected_model: ResMut<SelectedModel>,
    mut writer: EventWriter<LoadModel>,
) {
    if selected_model.0.is_empty() {
        selected_model.0 = MODELS[0].to_string();
    }

    bevy_egui::egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Click and drag to orbit camera");
        ui.label("Scroll to zoom camera");
        ui.label("Press 'e' to test export");
        ui.label("Press 'r' to re-load the scene");

        ui.separator();

        ComboBox::from_label("Loader")
            .selected_text(loader.0.to_string().as_str())
            .show_ui(ui, |ui| {
                for l in [GltfLoader::BevyGltf, GltfLoader::GltfKun] {
                    if ui
                        .selectable_label(
                            l.to_string().as_str() == loader.0.to_string().as_str(),
                            l.to_string().as_str(),
                        )
                        .clicked()
                    {
                        *loader = Loader(l);
                        exported.0.clear();
                        writer.send(LoadModel(selected_model.0.clone()));
                    }
                }
            });

        ComboBox::from_label("Model")
            .selected_text(selected_model.0.as_str())
            .show_ui(ui, |ui| {
                for model in MODELS {
                    if ui
                        .selectable_label(selected_model.0.as_str() == *model, *model)
                        .clicked()
                    {
                        selected_model.0 = model.to_string();
                        exported.0.clear();
                        writer.send(LoadModel(model.to_string()));
                    }
                }
            });
    });
}

fn load_model(
    asset_server: Res<AssetServer>,
    loader: Res<Loader>,
    mut events: EventReader<LoadModel>,
    mut gltf_events: EventReader<AssetEvent<Gltf>>,
    mut gltf_kun_events: EventReader<AssetEvent<GltfKun>>,
    mut handle: Local<GltfHandle>,
    mut writer: EventWriter<LoadScene>,
) {
    for event in events.read() {
        info!("Loading model {}", event.0);

        match loader.0 {
            GltfLoader::BevyGltf => {
                let h = asset_server.load::<Gltf>(event.0.clone());
                *handle = GltfHandle::Bevy(h);
            }
            GltfLoader::GltfKun => {
                let h = asset_server.load::<GltfKun>(event.0.clone());
                *handle = GltfHandle::GltfKun(h);
            }
        }

        writer.send(LoadScene(handle.clone()));
    }

    for event in gltf_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf loaded with dependencies");
            writer.send(LoadScene(handle.clone()));
        }
    }

    for event in gltf_kun_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf_kun loaded with dependencies");
            writer.send(LoadScene(handle.clone()));
        }
    }
}

fn load_scene(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_kun_assets: Res<Assets<GltfKun>>,
    loader: Res<Loader>,
    mut commands: Commands,
    mut events: EventReader<LoadScene>,
    scenes: Query<Entity, With<Handle<Scene>>>,
) {
    for event in events.read() {
        // Despawn previous scene.
        for entity in scenes.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let scene = match loader.0 {
            GltfLoader::BevyGltf => {
                let handle = match &event.0 {
                    GltfHandle::Bevy(handle) => handle,
                    _ => panic!("Invalid handle"),
                };

                let gltf = match gltf_assets.get(handle) {
                    Some(gltf) => gltf,
                    None => {
                        error!("Failed to get gltf asset");
                        return;
                    }
                };

                gltf.default_scene.clone().unwrap_or(gltf.scenes[0].clone())
            }
            GltfLoader::GltfKun => {
                let handle = match &event.0 {
                    GltfHandle::GltfKun(handle) => handle,
                    _ => panic!("Invalid handle"),
                };

                let gltf = match gltf_kun_assets.get(handle) {
                    Some(gltf) => gltf,
                    None => {
                        error!("Failed to get gltf_kun asset");
                        return;
                    }
                };

                gltf.default_scene.clone().unwrap_or(gltf.scenes[0].clone())
            }
        };

        commands.spawn(SceneBundle { scene, ..default() });
    }
}

fn export(
    mut export: EventWriter<GltfExport<DefaultExtensions>>,
    mut key_events: EventReader<ReceivedCharacter>,
    scene: Query<&Handle<Scene>>,
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
    mut writer: EventWriter<LoadModel>,
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

        writer.send(LoadModel(used_path));
    }
}

const TEMP_FOLDER: &str = "temp/bevy_gltf";

fn get_result(
    frame: Res<FrameCount>,
    mut exported_path: ResMut<ExportedPath>,
    mut exports: ResMut<Events<GltfExportResult>>,
    mut writer: EventWriter<LoadModel>,
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

        #[cfg(target_family = "wasm")]
        {
            // TODO
        }

        #[cfg(not(target_family = "wasm"))]
        {
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
            writer.send(LoadModel(file_path_str));
        }
    }
}

fn temp_file(frame: u32) -> String {
    format!("model_{}.glb", frame)
}
