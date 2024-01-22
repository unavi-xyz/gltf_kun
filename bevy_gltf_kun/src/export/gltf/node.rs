use bevy::prelude::*;
use gltf_kun::graph::{gltf::node, GraphNode};

use super::{CachedNode, ExportContext};

pub fn export_nodes(
    In(mut context): In<ExportContext>,
    nodes: Query<(&Transform, Option<&Name>, Option<&Children>)>,
) -> ExportContext {
    context.doc.scenes(&context.graph).iter().for_each(|scene| {
        let entity = context
            .scenes
            .iter()
            .find(|cached| cached.scene == *scene)
            .unwrap()
            .entity;

        let children = match nodes.get(entity) {
            Ok((_, _, Some(children))) => children,
            _ => return,
        };

        children.iter().for_each(|child| {
            let n = export_node(&mut context, &nodes, *child);
            scene.add_node(&mut context.graph, &n);
        });
    });

    context
}

fn export_node(
    context: &mut ExportContext,
    nodes: &Query<(&Transform, Option<&Name>, Option<&Children>)>,
    entity: Entity,
) -> node::Node {
    let mut node = context.doc.create_node(&mut context.graph);
    let weight = node.get_mut(&mut context.graph);

    let (transform, name, children) = nodes.get(entity).expect("Node not found");

    if let Some(name) = name {
        weight.name = Some(name.to_string());
    }

    weight.translation = transform.translation.to_array().into();
    weight.rotation = glam::Quat::from_array(transform.rotation.to_array());
    weight.scale = transform.scale.to_array().into();

    if let Some(children) = children {
        children.iter().for_each(|child| {
            let n = export_node(context, nodes, *child);
            node.add_child(&mut context.graph, &n);
        })
    }

    context.nodes.push(CachedNode { node, entity });

    node
}
