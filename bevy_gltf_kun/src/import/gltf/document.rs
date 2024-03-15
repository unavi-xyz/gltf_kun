use bevy::render::mesh::skinning::SkinnedMeshInverseBindposes;
use bevy::utils::HashSet;
use bevy::{asset::LoadContext, prelude::*, utils::HashMap};
use gltf_kun::graph::gltf::{Material, Node, Skin};
use gltf_kun::graph::{gltf::GltfDocument, Graph};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::skin::import_skin_matrices;
use super::{
    animation::{import_animation, paths_recur, AnimationImportError},
    node::GltfNode,
    scene::import_scene,
    texture::{get_linear_textures, load_texture, texture_label, TextureLoadError},
    GltfKun,
};

#[derive(Debug, Error)]
pub enum DocumentImportError {
    #[error(transparent)]
    TextureLoad(#[from] TextureLoadError),
    #[error(transparent)]
    Animation(#[from] AnimationImportError),
}

pub struct ImportContext<'a, 'b> {
    pub doc: &'a mut GltfDocument,
    pub gltf: &'a mut GltfKun,
    pub graph: &'a mut Graph,
    pub load_context: &'a mut LoadContext<'b>,

    pub nodes_handles: HashMap<Node, Handle<GltfNode>>,
    pub skin_matrices: HashMap<Skin, Handle<SkinnedMeshInverseBindposes>>,
    pub materials: HashMap<(Material, bool), Handle<StandardMaterial>>,
}

pub fn import_gltf_document<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
) -> Result<(), DocumentImportError> {
    // Load skins.
    for skin in context.doc.skins(context.graph) {
        match import_skin_matrices(context, skin) {
            Ok(handle) => {
                context.skin_matrices.insert(skin, handle);
            }
            Err(e) => {
                warn!("Failed to load skin matrices: {}", e);
            }
        }
    }

    // Load animations.
    let mut paths = HashMap::new();
    for scene in context.doc.scenes(context.graph) {
        for node in scene.nodes(context.graph) {
            paths_recur(context.doc, context.graph, &[], node, &mut paths, node);
        }
    }

    let mut animation_roots = HashSet::new();

    for animation in context.doc.animations(context.graph) {
        let (roots, handle) = import_animation(context, &paths, animation)?;
        animation_roots.extend(roots);
        context.gltf.animations.push(handle);
    }

    // Load textures.
    let linear_textures = get_linear_textures(context);

    for (i, texture) in context.doc.textures(context.graph).iter().enumerate() {
        if let Some(image) = texture.image(context.graph) {
            let is_srgb = !linear_textures.contains(texture);
            let texture = load_texture(context, *texture, image, is_srgb)?;
            let label = texture_label(i);
            let handle = context.load_context.add_labeled_asset(label, texture);
            context.gltf.images.insert(i, handle);
        }
    }

    // Load scenes.
    let default_scene = context.doc.default_scene(context.graph);

    for scene in context.doc.scenes(context.graph).into_iter() {
        let handle = import_scene::<E>(context, &animation_roots, scene);

        if Some(scene) == default_scene {
            context.gltf.default_scene = Some(handle.clone());
        }
    }

    // Load extensions.
    E::import_root(context);

    Ok(())
}
