use std::collections::HashMap;

use bevy::{prelude::*, render::mesh::morph::MorphBuildError};
use gltf_kun::graph::{
    gltf::{GltfDocument, Node},
    Graph, GraphNodeWeight,
};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::{
    document::ImportContext,
    mesh::{import_mesh, mesh_label, GltfMesh},
    primitive::primitive_label,
};

#[derive(Asset, Debug, TypePath)]
pub struct GltfNode {
    pub children: Vec<Handle<GltfNode>>,
    pub mesh: Option<Handle<GltfMesh>>,
    pub transform: Transform,
    pub extras: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Error)]
pub enum ImportNodeError {
    #[error(transparent)]
    MorphBuildEror(#[from] MorphBuildError),
}

pub fn import_node<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext<'_, '_>,
    node_entities: &mut HashMap<Handle<GltfNode>, Entity>,
    node_primitive_entities: &mut HashMap<Handle<GltfNode>, Vec<Entity>>,
    builder: &mut WorldChildBuilder,
    parent_world_transform: &Transform,
    n: &mut Node,
) -> Result<Handle<GltfNode>, ImportNodeError> {
    let index = context.doc.node_index(context.graph, *n).unwrap();
    let weight = n.get_mut(context.graph);

    let extras = weight.extras.take();
    let transform = Transform {
        translation: Vec3::from_array(weight.translation.to_array()),
        rotation: Quat::from_array(weight.rotation.to_array()),
        scale: Vec3::from_array(weight.scale.to_array()),
    };

    let world_transform = *parent_world_transform * transform;
    let is_scale_inverted = world_transform.scale.is_negative_bitmask().count_ones() & 1 == 1;

    let mut ent = builder.spawn(SpatialBundle::from_transform(transform));

    let name = node_name(context.doc, context.graph, *n);
    ent.insert(Name::new(name.clone()));

    let mut primitive_entities = Vec::new();

    let mesh = match n.mesh(context.graph) {
        Some(m) => {
            let (ents, mesh, morph_weights) =
                import_mesh::<E>(context, &mut ent, m, is_scale_inverted);

            primitive_entities.extend(ents);

            if let Some(weights) = morph_weights {
                let mesh_index = context.doc.mesh_index(context.graph, m).unwrap();
                let m_label = mesh_label(mesh_index);
                let p_label = primitive_label(&m_label, 0);
                let first_mesh = context.load_context.get_label_handle(p_label);
                let morph_weights = MorphWeights::new(weights, Some(first_mesh))?;
                ent.insert(morph_weights);
            }

            Some(mesh)
        }
        None => None,
    };

    let mut children = Vec::new();

    ent.with_children(|parent| {
        for c in n.children(context.graph).iter_mut() {
            match import_node::<E>(
                context,
                node_entities,
                node_primitive_entities,
                parent,
                &world_transform,
                c,
            ) {
                Ok(handle) => children.push(handle),
                Err(e) => {
                    warn!("Failed to import node: {}", e);
                    continue;
                }
            }
        }
    });

    let node = GltfNode {
        children,
        extras,
        mesh,
        transform,
    };

    let node_label = node_label(index);
    let handle = context
        .load_context
        .add_labeled_asset(node_label.clone(), node);

    context.gltf.named_nodes.insert(name, handle.clone());
    context.gltf.node_handles.insert(*n, handle.clone());
    context.gltf.nodes.insert(index, handle.clone());

    node_entities.insert(handle.clone(), ent.id());
    node_primitive_entities.insert(handle.clone(), primitive_entities);

    // Load extensions.
    E::import_node(context, &mut ent, *n);

    Ok(handle)
}

pub fn node_name(doc: &GltfDocument, graph: &Graph, node: Node) -> String {
    let weight = node.get(graph);
    weight
        .name
        .clone()
        .unwrap_or_else(|| node_label(doc.node_index(graph, node).unwrap()))
}

pub fn node_label(index: usize) -> String {
    format!("Node{}", index)
}
