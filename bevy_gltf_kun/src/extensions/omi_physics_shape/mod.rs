use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::{
    extensions::omi_physics_shape::OMIPhysicsShape,
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
};

use crate::import::gltf::document::ImportContext;

use super::{BevyExtensionExport, BevyExtensionImport};

impl BevyExtensionImport<GltfDocument> for OMIPhysicsShape {
    fn import_bevy(context: &mut ImportContext) {
        let ext = match context.doc.get_extension::<Self>(context.graph) {
            Some(ext) => ext,
            None => return,
        };
    }
}

impl BevyExtensionExport<GltfDocument> for OMIPhysicsShape {
    fn export_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {}
}
