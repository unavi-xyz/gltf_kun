//! Example glTF loading using test assets.

use std::{fmt::Display, path::Path};

use avian3d::prelude::*;
use bevy::{
    diagnostic::FrameCount,
    gltf::Gltf,
    input::keyboard::{Key, KeyboardInput},
    light::CascadeShadowConfigBuilder,
    prelude::*,
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui::ComboBox};
use bevy_gltf_kun::{
    GltfKunPlugin,
    export::gltf::{GltfExportEvent, GltfExportResult},
    import::gltf::{GltfKun, scene::GltfScene},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use gltf_kun::{extensions::DefaultExtensions, io::format::glb::GlbExport};

const ASSETS_DIR: &str = "assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const MODELS: &[&str] = &[
    "AnimatedMorphCube.glb",
    "AlphaBlendModeTest.glb",
    "AnimatedCube/AnimatedCube.gltf",
    "BoomBox.glb",
    "BoxInterleaved.glb",
    "BoxTextured.glb",
    "BoxTextured/BoxTextured.gltf",
    "BoxTexturedNonPowerOfTwo.glb",
    "DynamicBox.gltf",
    "InterpolationTest.glb",
    "MultiUVTest.glb",
    "NegativeScaleTest.glb",
    "OrientationTest.glb",
    "RecursiveSkeletons.glb",
    "RiggedFigure.glb",
    "RiggedSimple.glb",
    "SimpleSkin.gltf",
    "SimpleSparseAccessor.gltf",
    "TextureCoordinateTest.glb",
    "TextureSettingsTest.glb",
    "VertexColorTest.glb",
];

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            GltfKunPlugin::default(),
            PanOrbitCameraPlugin,
            PhysicsDebugPlugin,
            PhysicsPlugins::default(),
        ))
        .add_message::<LoadModel>()
        .add_message::<LoadScene>()
        .init_resource::<ExportedPath>()
        .init_resource::<LoadedKun>()
        .init_resource::<Loader>()
        .init_resource::<SelectedModel>()
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, run_ui)
        .add_systems(
            Update,
            (
                (load_scene, load_model, play_animations).chain(),
                export,
                get_result,
                reload,
            ),
        );
    }
}

#[derive(Message)]
struct LoadModel(String);

#[derive(Message)]
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

#[derive(Default, Resource)]
struct LoadedKun(Option<Handle<GltfKun>>);

fn setup(mut commands: Commands, mut writer: MessageWriter<LoadModel>) {
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(1.0, 2.0, 5.0),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 3.0),
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 100.0,
            first_cascade_far_bound: 8.0,
            ..default()
        }
        .build(),
    ));

    writer.write(LoadModel(MODELS[0].to_string()));
}

fn run_ui(
    mut contexts: EguiContexts,
    mut exported: ResMut<ExportedPath>,
    mut loader: ResMut<Loader>,
    mut pan_orbit_camera: Query<&mut PanOrbitCamera>,
    mut selected_model: ResMut<SelectedModel>,
    mut writer: MessageWriter<LoadModel>,
) {
    if selected_model.0.is_empty() {
        selected_model.0 = MODELS[0].to_string();
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    bevy_egui::egui::Window::new("Controls").show(ctx, |ui| {
        ui.label("Click and drag to orbit camera");
        ui.label("Scroll to zoom camera");
        #[cfg(not(target_family = "wasm"))]
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
                        writer.write(LoadModel(selected_model.0.clone()));
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
                        writer.write(LoadModel(model.to_string()));
                    }
                }
            });
    });

    for mut orbit in pan_orbit_camera.iter_mut() {
        orbit.enabled = !ctx.is_pointer_over_area();
    }
}

#[allow(clippy::too_many_arguments)]
fn load_model(
    asset_server: Res<AssetServer>,
    loader: Res<Loader>,
    mut events: MessageReader<LoadModel>,
    mut gltf_events: MessageReader<AssetEvent<Gltf>>,
    mut gltf_handle: Local<GltfHandle>,
    mut gltf_kun_events: MessageReader<AssetEvent<GltfKun>>,
    mut loaded_kun: ResMut<LoadedKun>,
    mut writer: MessageWriter<LoadScene>,
) {
    for event in events.read() {
        info!("Loading model {}", event.0);

        *gltf_handle = match loader.0 {
            GltfLoader::BevyGltf => {
                let h = asset_server.load::<Gltf>(event.0.clone());
                loaded_kun.0 = None;
                GltfHandle::Bevy(h)
            }
            GltfLoader::GltfKun => {
                let h = asset_server.load::<GltfKun>(event.0.clone());
                loaded_kun.0 = Some(h.clone());
                GltfHandle::GltfKun(h)
            }
        };

        writer.write(LoadScene(gltf_handle.clone()));
    }

    for event in gltf_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf loaded");
            writer.write(LoadScene(gltf_handle.clone()));
        }
    }

    for event in gltf_kun_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf_kun loaded");
            writer.write(LoadScene(gltf_handle.clone()));
        }
    }
}

fn load_scene(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_kun_assets: Res<Assets<GltfKun>>,
    loader: Res<Loader>,
    mut commands: Commands,
    mut events: MessageReader<LoadScene>,
    scenes: Query<Entity, With<SceneRoot>>,
    gltf_scenes: Res<Assets<GltfScene>>,
) {
    for event in events.read() {
        // Despawn previous scene.
        for entity in scenes.iter() {
            commands.entity(entity).despawn();
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
                        warn!("Failed to get gltf_kun asset");
                        return;
                    }
                };

                let gltf_scene_handle =
                    gltf.default_scene.clone().unwrap_or(gltf.scenes[0].clone());

                let gltf_scene = match gltf_scenes.get(&gltf_scene_handle) {
                    Some(scene) => scene,
                    None => {
                        error!("Failed to get gltf scene");
                        return;
                    }
                };

                gltf_scene.scene.clone()
            }
        };

        commands.spawn(SceneRoot(scene));
    }
}

fn play_animations(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_kun_assets: Res<Assets<GltfKun>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (entity, mut player) in players.iter_mut() {
        let mut graph = AnimationGraph::default();
        let mut animation_nodes = Vec::new();

        for (_, gltf) in gltf_assets.iter() {
            for clip in gltf.animations.iter() {
                animation_nodes.push(graph.add_clip(clip.clone(), 1.0, graph.root));
            }
        }

        for (_, gltf) in gltf_kun_assets.iter() {
            for clip in gltf.animations.iter() {
                animation_nodes.push(graph.add_clip(clip.clone(), 1.0, graph.root));
            }
        }

        for node in animation_nodes {
            player.play(node).repeat();
        }

        let handle = animation_graphs.add(graph);
        commands.entity(entity).insert(AnimationGraphHandle(handle));
    }
}

fn export(
    mut export: MessageWriter<GltfExportEvent<DefaultExtensions>>,
    mut key_events: MessageReader<KeyboardInput>,
    scene: Query<&SceneRoot>,
) {
    for event in key_events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        if event.logical_key != Key::Character("e".into()) {
            continue;
        }

        info!("Exporting scene");

        let handle = match scene.single() {
            Ok(handle) => handle,
            Err(e) => {
                error!("Failed to get scene: {}", e);
                return;
            }
        };

        export.write(GltfExportEvent::new(handle.0.clone()));
    }
}

fn reload(
    mut writer: MessageWriter<LoadModel>,
    mut key_events: MessageReader<KeyboardInput>,
    exported: Res<ExportedPath>,
    selected: Res<SelectedModel>,
) {
    for event in key_events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        if event.logical_key != Key::Character("r".into()) {
            continue;
        }

        let mut used_path = exported.0.clone();

        if used_path.is_empty() {
            used_path.clone_from(&selected.0);
        }

        info!("Reloading scene");

        writer.write(LoadModel(used_path));
    }
}

const TEMP_FOLDER: &str = "temp/bevy_gltf";

fn get_result(
    frame: Res<FrameCount>,
    mut exported_path: ResMut<ExportedPath>,
    mut exports: ResMut<Messages<GltfExportResult>>,
    mut writer: MessageWriter<LoadModel>,
) {
    for mut event in exports.drain() {
        let doc = match event.result {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to export from Bevy: {}", e),
        };

        let glb = match GlbExport::<DefaultExtensions>::export(&mut event.graph, &doc) {
            Ok(glb) => glb,
            Err(e) => panic!("Failed to export to glb: {}", e),
        };

        #[cfg(target_family = "wasm")]
        {
            // TODO: Exporting in wasm, not sure how to load exported glb into Bevy
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
            exported_path.0.clone_from(&file_path_str);

            info!("Writing glb to {}", file_path.display());

            let full_path = Path::new(CARGO_MANIFEST_DIR)
                .join(ASSETS_DIR)
                .join(file_path);

            std::fs::write(full_path, glb.0).expect("Failed to write glb");

            // Load glb
            writer.write(LoadModel(file_path_str));
        }
    }
}

fn temp_file(frame: u32) -> String {
    format!("model_{}.glb", frame)
}
