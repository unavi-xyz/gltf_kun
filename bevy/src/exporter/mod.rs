use bevy::prelude::*;

use self::utils::name_to_string;
use gltf_kun::{graph::NodeCover, node::Node};

mod utils;

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

            let mut data = scene.data();
            data.name = name_to_string(name);
            scene.set_data(data);

            let children = match children {
                Some(children) => children.to_vec(),
                None => Vec::new(),
            };

            children.iter().for_each(|entity| {
                let mut node = export_node(entity, &mut gltf, &nodes_query);
                scene.add_node(&mut node);
            })
        }
    }
}

fn export_node(
    entity: &Entity,
    gltf: &mut gltf_kun::Gltf,
    nodes_query: &Query<(&Transform, Option<&Name>, Option<&Children>)>,
) -> Node {
    let (transform, name, children) = match nodes_query.get(*entity) {
        Ok(node) => node,
        Err(_) => panic!("Node not found"),
    };

    let mut node = gltf.create_node();

    let mut data = node.data();
    data.name = name_to_string(name);
    data.translation = transform.translation.into();
    data.rotation = transform.rotation.into();
    data.scale = transform.scale.into();
    node.set_data(data);

    let children = match children {
        Some(children) => children.to_vec(),
        None => Vec::new(),
    };

    children
        .iter()
        .map(|ent| export_node(ent, gltf, nodes_query))
        .for_each(|mut child| node.add_child(&mut child));

    node
}
