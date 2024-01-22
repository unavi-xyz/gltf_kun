use bevy::prelude::*;
use gltf_kun::{
    extensions::{omi_physics_body::OMIPhysicsBody, DefaultExtensions, Extension},
    graph::{
        gltf::{document::GltfDocument, node::Node},
        Property,
    },
};

use crate::import::gltf::document::ImportContext;

#[cfg(feature = "omi_physics")]
pub mod omi_physics;

pub struct ExtensionsPlugin;

impl Plugin for ExtensionsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "omi_physics")]
        app.add_plugins(omi_physics::OMIPhysicsPlugin);
    }
}

pub trait NodeExtensionImport<D>: Extension {
    fn maybe_import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        if let Some(ext) = node.get_extension::<Self>(context.graph) {
            Self::import_node(context, entity, ext);
        }
    }

    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, ext: Self);
}

pub trait BevyImportExtensions<D> {
    fn process_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node);
}

impl BevyImportExtensions<GltfDocument> for DefaultExtensions {
    fn process_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {
        OMIPhysicsBody::maybe_import_node(context, entity, node)
    }
}
