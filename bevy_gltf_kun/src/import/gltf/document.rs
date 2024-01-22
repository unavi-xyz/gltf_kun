use bevy::asset::LoadContext;
use gltf_kun::graph::{gltf::document::GltfDocument, Graph};
use thiserror::Error;

use crate::extensions::BevyImportExtensions;

use super::{scene::import_scene, Gltf};

#[derive(Debug, Error)]
pub enum DocumentImportError {}

pub struct ImportContext<'a, 'b> {
    pub graph: &'a mut Graph,
    pub doc: &'a mut GltfDocument,
    pub gltf: &'a mut Gltf,
    pub load_context: &'a mut LoadContext<'b>,
}

pub fn import_gltf_document<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
) -> Result<(), DocumentImportError> {
    let default_scene = context.doc.default_scene(context.graph);

    for scene in context.doc.scenes(context.graph) {
        let handle = import_scene::<E>(context, scene)?;

        if Some(scene) == default_scene {
            context.gltf.default_scene = Some(handle);
        }
    }

    Ok(())
}
