use bevy::asset::LoadContext;
use gltf_kun::graph::{gltf::document::GltfDocument, Graph};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::{
    image::{import_images, ImageImportError},
    material::import_material,
    scene::import_scene,
    GltfKun,
};

#[derive(Debug, Error)]
pub enum DocumentImportError {
    #[error("Failed to import image: {0}")]
    Image(#[from] ImageImportError),
}

pub struct ImportContext<'a, 'b> {
    pub doc: &'a mut GltfDocument,
    pub gltf: &'a mut GltfKun,
    pub graph: &'a mut Graph,
    pub load_context: &'a mut LoadContext<'b>,
}

pub fn import_gltf_document<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
) -> Result<(), DocumentImportError> {
    import_images::<E>(context)?;

    for material in context.doc.materials(context.graph) {
        if let Ok(handle) = import_material::<E>(context, material) {
            context.gltf.materials.push(handle);
        }
    }

    let default_scene = context.doc.default_scene(context.graph);

    for scene in context.doc.scenes(context.graph) {
        if let Ok(handle) = import_scene::<E>(context, scene) {
            if Some(scene) == default_scene {
                context.gltf.default_scene = Some(handle.clone());
            }

            context.gltf.scenes.push(handle);
        }
    }

    Ok(())
}
