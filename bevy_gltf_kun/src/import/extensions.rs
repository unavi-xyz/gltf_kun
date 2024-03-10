use bevy::prelude::*;
use gltf_kun::{
    extensions::{DefaultExtensions, Extension},
    graph::{
        gltf::{document::GltfDocument, node::Node},
        Property,
    },
};

use crate::import::gltf::document::ImportContext;

pub trait NodeExtensionImport<D>: Extension {
    fn maybe_import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        if let Some(ext) = node.get_extension::<Self>(context.graph) {
            Self::import_node(context, entity, ext);
        }
    }

    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self);
}

pub trait BevyImportExtensions<D> {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node);
}

impl BevyImportExtensions<GltfDocument> for DefaultExtensions {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        #[cfg(feature = "omi_physics")]
        {
            gltf_kun::extensions::omi_physics_body::OMIPhysicsBody::maybe_import_node(
                context, entity, node,
            )
        }
    }
}
