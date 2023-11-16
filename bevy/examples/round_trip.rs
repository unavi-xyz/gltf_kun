use bevy::prelude::*;
use bevy_gltf_kun::ExportResult;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
            bevy_gltf_kun::GltfExportPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (export_scene, read_result))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        ..default()
    });

    let scene = commands.spawn(SceneBundle::default()).id();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                ..default()
            })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.5, 1.0, 0.5))),
            ..default()
        })
        .set_parent(scene);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(StandardMaterial::from(Color::rgb(1.0, 0.5, 0.5))),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .set_parent(scene);
}

/// Send an export event after a delay
fn export_scene(
    mut event_writer: EventWriter<bevy_gltf_kun::ExportScene>,
    scenes: Query<Entity, With<Handle<Scene>>>,
    time: Res<Time>,
    mut exported: Local<bool>,
) {
    if *exported {
        return;
    }

    if time.elapsed_seconds() < 3.0 {
        return;
    }

    for scene in scenes.iter() {
        event_writer.send(bevy_gltf_kun::ExportScene {
            scenes: vec![scene],
            format: bevy_gltf_kun::ExportFormat::Binary,
        });
    }

    *exported = true;
}

const MODEL_PATH: &str = "temp/round_trip.glb";

fn read_result(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut results: EventReader<ExportResult>,
    mut scenes: Query<Entity, With<Handle<Scene>>>,
) {
    for event in results.read() {
        match event {
            ExportResult::Binary(bytes) => {
                // Write glb to disk
                std::fs::write(format!("assets/{}", MODEL_PATH), bytes).unwrap();

                // Clear scene
                for scene in scenes.iter_mut() {
                    commands.entity(scene).despawn_recursive();
                }

                // Load glb back into bevy
                let scene = asset_server.load(format!("{}#Scene0", MODEL_PATH));
                commands.spawn(SceneBundle { scene, ..default() });
            }
            _ => panic!("Not a binary export result!"),
        }
    }
}
