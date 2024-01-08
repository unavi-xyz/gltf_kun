use bevy::prelude::*;

use super::ExportContext;

pub fn export_nodes(
    In(context): In<ExportContext>,
    // nodes: Query<(&Transform, Option<&Children>)>
) {
    // let mut node = node::Node::new(graph);
    // let weight = node.get_mut(graph);
    //
    // let (transform, children) = nodes.get(entity)?;
    //
    // if let Ok(name) = names.get(entity) {
    //     weight.name = Some(name.to_string());
    // }
    //
    // weight.translation = transform.translation.to_array().into();
    // weight.rotation = glam::Quat::from_array(transform.rotation.to_array());
    // weight.scale = transform.scale.to_array().into();
    //
    // let mut child_ents = BTreeMap::<node::Node, Entity>::new();
    //
    // if let Some(children) = children {
    //     children.iter().for_each(|child| {
    //         match export_node(graph, context, names, mesh_assets, meshes, nodes, *child) {
    //             Ok(node) => {
    //                 child_ents.insert(node, *child);
    //                 node.add_child(graph, &node)
    //             }
    //             Err(_) => {
    //                 warn!("Node not found: {:?}", child);
    //             }
    //         }
    //     })
    // }
    //
    // if let Some(mesh) = export_mesh(
    //     graph,
    //     context,
    //     mesh_assets,
    //     meshes,
    //     names,
    //     &child_ents,
    //     entity,
    // ) {
    //     node.set_mesh(graph, Some(&mesh));
    // }
    //
    // Ok(node)
}
