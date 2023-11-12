use bevy::prelude::*;

pub fn export_gltf(
    mut events: EventReader<super::ExportScene>,
    meshes: Res<Assets<Mesh>>,
    scenes: Query<(Option<&Name>, Option<&Children>), With<Handle<Scene>>>,
    nodes_query: Query<(&Transform, Option<&Name>, Option<&Children>)>,
    meshes_query: Query<(&Handle<Mesh>, Option<&Name>)>,
) {
    for event in events.read() {
        let mut gltf = gltf_kun::Gltf::default();

        for scene in &event.scenes {
            let (name, children) = match scenes.get(*scene) {
                Ok(scene) => scene,
                Err(_) => {
                    error!("Scene not found");
                    continue;
                }
            };

            let mut scene = gltf.create_scene();

            let children = match children {
                Some(children) => children.to_vec(),
                None => Vec::new(),
            };

            children.iter().for_each(|entity| {
                let (transform, name, children) = match nodes_query.get(*entity) {
                    Ok(node) => node,
                    Err(_) => {
                        error!("Node not found");
                        return;
                    }
                };

                // let mut node = gltf.create_node();
                // scene.add_node(&mut node);
            })
        }
    }
}
