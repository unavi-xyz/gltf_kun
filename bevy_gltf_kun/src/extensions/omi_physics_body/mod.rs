use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::{
    extensions::omi_physics_body::OMIPhysicsBody,
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
};

use super::{BevyExtensionExport, BevyExtensionImport};

impl BevyExtensionImport<GltfDocument> for OMIPhysicsBody {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {
        doc.nodes(graph)
            .iter()
            .filter_map(|n| n.get_extension::<Self>(graph))
            .for_each(|ext| {
                let weight = ext.read(graph);
                println!("BevyOMIPhysicsBody::import_bevy: {:?}", weight);
            });
    }
}

impl BevyExtensionExport<GltfDocument> for OMIPhysicsBody {
    fn export_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {}
}
