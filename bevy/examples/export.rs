use bevy::prelude::*;
use bevy_gltf_kun::ExportResult;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_gltf_kun::GltfExportPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, read_result)
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
        scenes: vec![scene],
        format: bevy_gltf_kun::ExportFormat::Standard,
    });
}

fn read_result(mut results: EventReader<ExportResult>) {
    for event in results.read() {
        match event {
            ExportResult::Standard { root, binary } => {
                let string_pretty = gltf::json::serialize::to_string_pretty(&root).unwrap();
                info!("root: {}", string_pretty);

                let string = gltf::json::serialize::to_string(&root).unwrap();
                let size = string.len() + binary.len();
                info!("export size: {}", format_byte_size(size));
            }
            ExportResult::Binary(bytes) => {
                info!("exported size: {}", format_byte_size(bytes.len()));
            }
        }
    }
}

fn format_byte_size(size: usize) -> String {
    let mut size = size as f32;
    let mut unit = "B";
    if size > 1024.0 {
        size /= 1024.0;
        unit = "KB";
    }
    if size > 1024.0 {
        size /= 1024.0;
        unit = "MB";
    }
    if size > 1024.0 {
        size /= 1024.0;
        unit = "GB";
    }
    format!("{:.2}{}", size, unit)
}
