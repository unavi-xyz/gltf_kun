use bevy::{asset::LoadContext, prelude::*, render::render_resource::Face};
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
    is_scale_inverted: bool,
) -> Handle<StandardMaterial> {
    let index = context.doc.material_index(context.graph, m).unwrap();
    let weight = m.get(context.graph);
    let label = material_label(index, is_scale_inverted);

    context
        .load_context
        .labeled_asset_scope(label, |load_context| {
            let alpha_mode = match &weight.alpha_mode {
                AlphaMode::Blend => bevy::prelude::AlphaMode::Blend,
                AlphaMode::Mask => bevy::prelude::AlphaMode::Mask(weight.alpha_cutoff.0),
                AlphaMode::Opaque => bevy::prelude::AlphaMode::Opaque,
            };

            let cull_mode = if weight.double_sided {
                None
            } else if is_scale_inverted {
                Some(Face::Front)
            } else {
                Some(Face::Back)
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
                base_color: Color::rgba_linear_from_array(weight.base_color_factor),
                base_color_texture,
                cull_mode,
                double_sided: weight.double_sided,
                emissive: Color::rgb_linear_from_array(weight.emissive_factor),
                emissive_texture,
                metallic: weight.metallic_factor,
                metallic_roughness_texture,
                normal_map_texture,
                occlusion_texture,
                perceptual_roughness: weight.roughness_factor,
                ..default()
            }
        })
}

const DEFAULT_MATERIAL_LABEL: &str = "MaterialDefault";

pub fn default_material(context: &mut ImportContext) -> Handle<StandardMaterial> {
    if context
        .load_context
        .has_labeled_asset(DEFAULT_MATERIAL_LABEL)
    {
        context
            .load_context
            .get_label_handle(DEFAULT_MATERIAL_LABEL)
    } else {
        context
            .load_context
            .labeled_asset_scope(DEFAULT_MATERIAL_LABEL.to_string(), |_| StandardMaterial {
                metallic: 1.0,
                perceptual_roughness: 1.0,
                ..default()
            })
    }
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

    let index = doc.texture_index(graph, info)?;
    let label = texture_label(index);

    Some(load_context.get_label_handle(&label))
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index)
}

fn material_label(index: usize, is_scale_inverted: bool) -> String {
    if is_scale_inverted {
        format!("Material{}(inverted)", index)
    } else {
        format!("Material{}", index)
    }
}
