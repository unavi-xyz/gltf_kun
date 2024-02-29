use bevy::prelude::*;
use gltf_kun::graph::{gltf::node, GraphNodeWeight};

use crate::import::gltf::node::node_label;

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

        children.iter().for_each(|c| {
            let (transform, name, grandchildren) = nodes.get(*c).expect("Node not found");

            if transform == &Transform::default() && name.is_none() {
                // Assume this is an empty root node, and skip it.
                // This is a bit of a hack, but helps keep consistency between import and export.
                debug!("Skipping empty root node");

                let grandchildren = match grandchildren {
                    Some(children) => children,
                    None => return,
                };

                for child in grandchildren {
                    let n = export_node(&mut context, &nodes, *child);
                    scene.add_node(&mut context.graph, n);
                }
            } else {
                let n = export_node(&mut context, &nodes, *c);
                scene.add_node(&mut context.graph, n);
            }
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
        let end_num: usize = name
            .to_string()
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap_or(0);

        let generated_name = node_label(end_num);

        // If the name is a generated node name, don't export it.
        // This may catch false positives, but particularly around animations it can lead to
        // conflicting names upon re-import.
        if name.to_string() != generated_name {
            weight.name = Some(name.to_string());
        }
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
