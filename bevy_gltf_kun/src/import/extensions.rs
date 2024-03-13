use bevy::prelude::*;
use gltf_kun::{
    extensions::{DefaultExtensions, Extension},
    graph::{
        gltf::{GltfDocument, Node, Primitive},
        Extensions,
    },
};

use crate::import::gltf::document::ImportContext;

pub trait NodeExtensionImport<D>: Extension {
    fn maybe_import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        if let Some(ext) = node.get_extension::<Self>(context.graph) {
            Self::import_node(context, entity, ext);
        }
    }

    /// Called when a node with this extension is imported.
    /// This is called while the tree is being traversed, so the node's children may not have been imported yet.
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self);
}

pub trait PrimitiveExtensionImport<D>: Extension {
    fn maybe_import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    ) {
        if let Some(ext) = primitive.get_extension::<Self>(context.graph) {
            Self::import_primitive(context, entity, ext);
        }
    }

    fn import_primitive(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self);
}

pub trait RootExtensionImport<D: Extensions>: Extension {
    fn maybe_import_root(context: &mut ImportContext) {
        if let Some(ext) = context.doc.get_extension::<Self>(context.graph) {
            Self::import_root(context, ext);
        }
    }

    fn import_root(context: &mut ImportContext, ext: Self);
}

pub trait BevyImportExtensions<D> {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node);
    fn import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    );
    fn import_root(context: &mut ImportContext);
}

impl BevyImportExtensions<GltfDocument> for DefaultExtensions {
    fn import_primitive(
        _context: &mut ImportContext,
        _entity: &mut EntityWorldMut,
        _primitive: Primitive,
    ) {
    }

    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        #[cfg(feature = "omi_physics")]
        {
            gltf_kun::extensions::omi_physics_body::OmiPhysicsBody::maybe_import_node(
                context, entity, node,
            )
        }
    }

    fn import_root(_context: &mut ImportContext) {}
}
