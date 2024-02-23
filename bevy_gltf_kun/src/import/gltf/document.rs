use bevy::asset::LoadContext;
use bevy::prelude::*;
use gltf_kun::graph::{gltf::GltfDocument, Graph};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::{
    image::{get_texture_infos, load_texture, texture_label, ImageImportError},
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
    // Load textures.
    for (i, (info, is_srgb)) in get_texture_infos(context).iter().enumerate() {
        if let Some(image) = info.image(context.graph) {
            let texture = load_texture(context, *info, image, *is_srgb)?;
            let label = texture_label(i);
            let handle = context.load_context.add_labeled_asset(label, texture);
            context.gltf.images.insert(i, handle);
        }
    }

    // Load materials.
    for (i, material) in context.doc.materials(context.graph).into_iter().enumerate() {
        match import_material::<E>(context, material) {
            Ok(handle) => context.gltf.materials.insert(i, handle),
            Err(e) => warn!("Failed to import material: {}", e),
        };
    }

    // Load scenes.
    let default_scene = context.doc.default_scene(context.graph);

    for (i, scene) in context.doc.scenes(context.graph).into_iter().enumerate() {
        match import_scene::<E>(context, scene) {
            Ok(handle) => {
                if Some(scene) == default_scene {
                    context.gltf.default_scene = Some(handle.clone());
                }

                context.gltf.scenes.insert(i, handle);
            }
            Err(e) => warn!("Failed to import scene: {}", e),
        }
    }

    Ok(())
}
