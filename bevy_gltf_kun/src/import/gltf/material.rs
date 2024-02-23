use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::graph::{
    gltf::{material::AlphaMode, GltfDocument, Material, TextureInfo},
    Graph, GraphNodeWeight,
};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum MaterialImportError {}

pub fn import_material<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
    m: Material,
) -> Result<Handle<StandardMaterial>, MaterialImportError> {
    let index = context
        .doc
        .materials(context.graph)
        .iter()
        .position(|x| *x == m)
        .unwrap();
    let weight = m.get(context.graph);
    let label = material_label(index);

    let handle = context
        .load_context
        .labeled_asset_scope(label, |load_context| {
            let alpha_mode = match &weight.alpha_mode {
                AlphaMode::Blend => bevy::prelude::AlphaMode::Blend,
                AlphaMode::Mask => bevy::prelude::AlphaMode::Mask(weight.alpha_cutoff.0),
                AlphaMode::Opaque => bevy::prelude::AlphaMode::Opaque,
            };

            let base_color_texture = texture_handle(
                context.doc,
                context.graph,
                load_context,
                m.base_color_texture_info(context.graph),
            );

            let emissive_texture = texture_handle(
                context.doc,
                context.graph,
                load_context,
                m.emissive_texture_info(context.graph),
            );

            let metallic_roughness_texture = texture_handle(
                context.doc,
                context.graph,
                load_context,
                m.metallic_roughness_texture_info(context.graph),
            );

            let normal_map_texture = texture_handle(
                context.doc,
                context.graph,
                load_context,
                m.normal_texture_info(context.graph),
            );

            let occlusion_texture = texture_handle(
                context.doc,
                context.graph,
                load_context,
                m.occlusion_texture_info(context.graph),
            );

            StandardMaterial {
                alpha_mode,
                base_color: Color::rgba_from_array(weight.base_color_factor),
                base_color_texture,
                double_sided: weight.double_sided,
                emissive: Color::rgb_from_array(weight.emissive_factor),
                emissive_texture,
                metallic: weight.metallic_factor,
                metallic_roughness_texture,
                normal_map_texture,
                occlusion_texture,
                ..default()
            }
        });

    Ok(handle)
}

fn texture_handle(
    doc: &mut GltfDocument,
    graph: &Graph,
    load_context: &mut LoadContext,
    info: Option<TextureInfo>,
) -> Option<Handle<Image>> {
    let info = match info {
        Some(info) => info,
        None => return None,
    };

    let image = match info.image(graph) {
        Some(image) => image,
        None => return None,
    };

    let image_index = doc.images(graph).iter().position(|x| *x == image).unwrap();
    let label = texture_label(image_index);

    Some(load_context.get_label_handle(&label))
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index,)
}

fn material_label(index: usize) -> String {
    format!("Material{}", index)
}
