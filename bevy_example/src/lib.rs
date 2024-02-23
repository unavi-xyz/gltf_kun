use std::{fmt::Display, path::Path};

use bevy::{core::FrameCount, gltf::Gltf, pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_egui::{egui::ComboBox, EguiContexts, EguiPlugin};
use bevy_gltf_kun::{
    export::gltf::{GltfExport, GltfExportResult},
    import::{gltf::GltfKun, graph::GltfGraph},
    GltfKunPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_xpbd_3d::prelude::*;
use egui_graphs::{
    DefaultEdgeShape, DefaultNodeShape, Graph, GraphView, SettingsInteraction, SettingsStyle,
};
use gltf_kun::{
    extensions::DefaultExtensions,
    graph::{Edge, Weight},
    io::format::glb::GlbIO,
};

use crate::graph::{create_graph, GraphSettings};

pub mod graph;

const ASSETS_DIR: &str = "assets";
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const MODELS: &[&str] = &[
    "AlphaBlendModeTest.glb",
    "BoxTextured.glb",
    "BoxTextured/BoxTextured.gltf",
    "BoxTexturedNonPowerOfTwo.glb",
    "DynamicBox.gltf",
    "OrientationTest.glb",
    "SimpleSkin.gltf",
    "SimpleSparseAccessor.gltf",
    "SimpleTexture.gltf",
    "TextureCoordinateTest.glb",
    "TextureSettingsTest.glb",
    "VertexColorTest.glb",
];

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GltfKunPlugin::<DefaultExtensions>::default(),
            EguiPlugin,
            PanOrbitCameraPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_event::<LoadModel>()
        .add_event::<LoadScene>()
        .init_resource::<ExportedPath>()
        .init_resource::<LoadedGraph>()
        .init_resource::<Loader>()
        .init_resource::<SelectedModel>()
        .init_resource::<GraphSet>()
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
        );
    }
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

#[derive(Default, Resource)]
struct LoadedGraph(Option<Graph<Weight, Edge>>);

#[derive(Default, Resource)]
struct GraphSet(pub GraphSettings);

fn setup(mut commands: Commands, mut writer: EventWriter<LoadModel>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 5.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 3.0),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 100.0,
            first_cascade_far_bound: 8.0,
            ..default()
        }
        .build(),

        ..default()
    });

    writer.send(LoadModel(MODELS[1].to_string()));
}

#[allow(clippy::too_many_arguments)]
fn ui(
    mut contexts: EguiContexts,
    mut exported: ResMut<ExportedPath>,
    mut loaded_graph: ResMut<LoadedGraph>,
    mut loader: ResMut<Loader>,
    mut selected_model: ResMut<SelectedModel>,
    mut writer: EventWriter<LoadModel>,
    mut pan_orbit_camera: Query<&mut PanOrbitCamera>,
    mut graph_settings: ResMut<GraphSet>,
) {
    if selected_model.0.is_empty() {
        selected_model.0 = MODELS[1].to_string();
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

        ui.separator();

        ui.collapsing("Graph settings", |ui| {
            if ui
                .checkbox(&mut graph_settings.0.enable_accessors, "Enable accessors")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_buffers, "Enable buffers")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_document, "Enable document")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_images, "Enable images")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(
                    &mut graph_settings.0.enable_texture_infos,
                    "Enable texture infos",
                )
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_materials, "Enable materials")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_primitives, "Enable primitives")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_meshes, "Enable meshes")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_nodes, "Enable nodes")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }

            if ui
                .checkbox(&mut graph_settings.0.enable_scenes, "Enable scenes")
                .clicked()
            {
                writer.send(LoadModel(selected_model.0.clone()));
            }
        });

        if let Some(graph) = loaded_graph.0.iter_mut().next() {
            let interaction_settings = &SettingsInteraction::new()
                .with_dragging_enabled(true)
                .with_node_clicking_enabled(true);

            let style_settings = &SettingsStyle::new().with_labels_always(true);

            ui.add(
                &mut GraphView::<_, _, _, _, DefaultNodeShape, DefaultEdgeShape>::new(graph)
                    .with_styles(style_settings)
                    .with_interactions(interaction_settings),
            );
        }
    });

    let ctx = contexts.ctx_mut();

    for mut orbit in pan_orbit_camera.iter_mut() {
        orbit.enabled = !ctx.is_pointer_over_area();
    }
}

#[allow(clippy::too_many_arguments)]
fn load_model(
    asset_server: Res<AssetServer>,
    graph_settings: Res<GraphSet>,
    graphs: Res<Assets<GltfGraph>>,
    loader: Res<Loader>,
    mut events: EventReader<LoadModel>,
    mut gltf_events: EventReader<AssetEvent<Gltf>>,
    mut gltf_handle: Local<GltfHandle>,
    mut gltf_kun_events: EventReader<AssetEvent<GltfKun>>,
    mut graph_events: EventReader<AssetEvent<GltfGraph>>,
    mut graph_handle: Local<Handle<GltfGraph>>,
    mut loaded_graph: ResMut<LoadedGraph>,
    mut writer: EventWriter<LoadScene>,
) {
    for event in events.read() {
        info!("Loading model {}", event.0);

        *graph_handle = asset_server.load::<GltfGraph>(event.0.clone());

        let graph = graphs
            .get(graph_handle.clone())
            .map(|g| create_graph(g, &graph_settings.0));
        *loaded_graph = LoadedGraph(graph);

        *gltf_handle = match loader.0 {
            GltfLoader::BevyGltf => {
                let h = asset_server.load::<Gltf>(event.0.clone());
                GltfHandle::Bevy(h)
            }
            GltfLoader::GltfKun => {
                let h = asset_server.load::<GltfKun>(event.0.clone());
                GltfHandle::GltfKun(h)
            }
        };

        writer.send(LoadScene(gltf_handle.clone()));
    }

    for event in graph_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Graph loaded");
            let graph = graphs
                .get(graph_handle.clone())
                .map(|g| create_graph(g, &graph_settings.0));
            *loaded_graph = LoadedGraph(graph);
        }
    }

    for event in gltf_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf loaded");
            writer.send(LoadScene(gltf_handle.clone()));
        }
    }

    for event in gltf_kun_events.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            info!("Gltf_kun loaded");
            writer.send(LoadScene(gltf_handle.clone()));
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
