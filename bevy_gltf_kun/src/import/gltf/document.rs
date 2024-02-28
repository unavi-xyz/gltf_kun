use bevy::prelude::*;
use bevy::{asset::LoadContext, utils::HashMap};
use gltf_kun::graph::{gltf::GltfDocument, Graph};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::animation::paths_recur;
use super::{
    animation::{import_animation, AnimationImportError},
    material::import_material,
    scene::import_scene,
    texture::{get_linear_textures, load_texture, texture_label, TextureLoadError},
    GltfKun,
};

#[derive(Debug, Error)]
pub enum DocumentImportError {
    #[error("Failed to load texture: {0}")]
    TextureLoad(#[from] TextureLoadError),
    #[error("Failed to load animation: {0}")]
    Animation(#[from] AnimationImportError),
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
    // Load animations.
    let mut animation_paths = HashMap::new();
    for scene in context.doc.scenes(context.graph) {
        for node in scene.nodes(context.graph) {
            paths_recur(
                context.doc,
                context.graph,
                Vec::new(),
                node,
                &mut animation_paths,
                node,
            );
        }
    }

    for animation in context.doc.animations(context.graph) {
        let handle = import_animation(context, &animation_paths, animation)?;
        context.gltf.animations.push(handle);
    }

    // Load textures.
    let linear_textures = get_linear_textures(context);

    for (i, texture_info) in context.doc.textures(context.graph).iter().enumerate() {
        if let Some(image) = texture_info.image(context.graph) {
            let is_srgb = !linear_textures.contains(texture_info);
            let texture = load_texture(context, *texture_info, image, is_srgb)?;
            let label = texture_label(i);
            let handle = context.load_context.add_labeled_asset(label, texture);
            context.gltf.images.insert(i, handle);
        }
    }

    // Load materials.
    for (i, material) in context.doc.materials(context.graph).into_iter().enumerate() {
        let handle = import_material::<E>(context, material);
        context.gltf.materials.insert(i, handle);
    }

    // Load scenes.
    let default_scene = context.doc.default_scene(context.graph);

    for (i, scene) in context.doc.scenes(context.graph).into_iter().enumerate() {
        let handle = import_scene::<E>(context, &animation_paths, scene);

        if Some(scene) == default_scene {
            context.gltf.default_scene = Some(handle.clone());
        }

        context.gltf.scenes.insert(i, handle);
    }

    Ok(())
}
