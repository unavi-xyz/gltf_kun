use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_gltf_kun::GltfExportPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut event_writer: EventWriter<bevy_gltf_kun::ExportScene>,
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

    event_writer.send(bevy_gltf_kun::ExportScene {
        scene,
        format: bevy_gltf_kun::ExportFormat::default(),
    });
}
