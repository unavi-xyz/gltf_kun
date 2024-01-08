use std::collections::BTreeMap;

use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::graph::gltf::node;

use super::{scene::SceneNodes, CachedNode, ExportContext};

pub fn export_nodes(
    In((mut context, scene_nodes)): In<(ExportContext, SceneNodes)>,
    nodes: Query<(&Transform, Option<&Name>, Option<&Children>)>,
) -> ExportContext {
    scene_nodes.iter().for_each(|(scene, children)| {
        children
            .iter()
            .for_each(|child| match export_node(&mut context, &nodes, *child) {
                Ok(node) => {
                    scene.add_node(&mut context.doc.0, &node);
                }
                Err(_) => {
                    warn!("Node not found: {:?}", child);
                }
            })
    });

    context
}

fn export_node(
    context: &mut ExportContext,
    nodes: &Query<(&Transform, Option<&Name>, Option<&Children>)>,
    entity: Entity,
) -> Result<node::Node> {
    let mut node = node::Node::new(&mut context.doc.0);
    let weight = node.get_mut(&mut context.doc.0);

    let (transform, name, children) = nodes.get(entity)?;

    if let Some(name) = name {
        weight.name = Some(name.to_string());
    }

    weight.translation = transform.translation.to_array().into();
    weight.rotation = glam::Quat::from_array(transform.rotation.to_array());
    weight.scale = transform.scale.to_array().into();

    let mut child_ents = BTreeMap::<node::Node, Entity>::new();

    if let Some(children) = children {
        children
            .iter()
            .for_each(|child| match export_node(context, nodes, *child) {
                Ok(node) => {
                    child_ents.insert(node, *child);
                    node.add_child(&mut context.doc.0, &node)
                }
                Err(_) => {
                    warn!("Node not found: {:?}", child);
                }
            })
    }

    context.nodes.push(CachedNode { node, entity });

    Ok(node)
}
