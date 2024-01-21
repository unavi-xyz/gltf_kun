use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::{
    extensions::omi_physics_body::OMIPhysicsBody,
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
};

use crate::import::gltf::document::ImportContext;

use super::{BevyExtensionExport, BevyExtensionImport};

impl BevyExtensionImport<GltfDocument> for OMIPhysicsBody {
    fn import_bevy(context: &mut ImportContext) {
        context
            .doc
            .nodes(context.graph)
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.get_extension::<Self>(context.graph).map(|e| (i, e)))
            .for_each(|(i, ext)| {});
    }
}

impl BevyExtensionExport<GltfDocument> for OMIPhysicsBody {
    fn export_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {}
}
