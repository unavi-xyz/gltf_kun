use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::{
    extensions::omi_physics_shape::OMIPhysicsShape,
    graph::{gltf::document::GltfDocument, ByteNode, Graph, Property},
};

use super::{BevyExtensionExport, BevyExtensionImport};

impl BevyExtensionImport<GltfDocument> for OMIPhysicsShape {
    fn import_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {}
}

impl BevyExtensionExport<GltfDocument> for OMIPhysicsShape {
    fn export_bevy(graph: &mut Graph, doc: &mut GltfDocument, load_context: &mut LoadContext) {}
}
