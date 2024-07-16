use bevy::prelude::*;
use gltf_kun::{
    extensions::{DefaultExtensions, Extension},
    graph::{
        gltf::{GltfDocument, Material, Node, Primitive, Scene},
        Extensions,
    },
};

use crate::import::gltf::document::ImportContext;

pub trait NodeExtensionImport<D>: Extension {
    fn try_import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        if let Some(ext) = node.get_extension::<Self>(context.graph) {
            Self::import_node(context, entity, ext);
        }
    }

    /// Hook for nodes with this extension.
    /// Called while the tree is being traversed, so the node's children may not have been imported yet.
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self);
}

pub trait BevyExtensionImport<D> {
    fn import_material(
        context: &mut ImportContext,
        standard_material: &mut StandardMaterial,
        material: Material,
    );
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node);
    fn import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    );
    fn import_root(context: &mut ImportContext);
    fn import_scene(context: &mut ImportContext, scene: Scene, world: &mut World);
}

impl BevyExtensionImport<GltfDocument> for DefaultExtensions {
    fn import_material(
        _context: &mut ImportContext,
        _standard_material: &mut StandardMaterial,
        _material: Material,
    ) {
    }

    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        #[cfg(feature = "omi_physics")]
        {
            gltf_kun::extensions::omi_physics_body::OmiPhysicsBody::try_import_node(
                context, entity, node,
            )
        }
    }

    fn import_primitive(
        _context: &mut ImportContext,
        _entity: &mut EntityWorldMut,
        _primitive: Primitive,
    ) {
    }
    fn import_root(_context: &mut ImportContext) {}
    fn import_scene(_context: &mut ImportContext, _scene: Scene, _world: &mut World) {}
}
