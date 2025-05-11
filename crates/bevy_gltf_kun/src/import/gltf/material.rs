use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
    render::render_resource::Face,
};
use gltf_kun::graph::{
    Graph, GraphNodeWeight,
    gltf::{GltfDocument, Material, Texture, material::AlphaMode},
};
use thiserror::Error;

use crate::import::extensions::BevyExtensionImport;

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum MaterialImportError {}

pub fn import_material<E: BevyExtensionImport<GltfDocument>>(
    context: &mut ImportContext,
    m: Material,
    is_scale_inverted: bool,
) -> Handle<StandardMaterial> {
    let index = context.doc.material_index(context.graph, m).unwrap();
    let weight = m.get(context.graph);
    let label = material_label(index, is_scale_inverted);

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
        context.load_context,
        m.base_color_texture(context.graph),
    );

    let emissive_texture = texture_handle(
        context.doc,
        context.graph,
        context.load_context,
        m.emissive_texture(context.graph),
    );

    let metallic_roughness_texture = texture_handle(
        context.doc,
        context.graph,
        context.load_context,
        m.metallic_roughness_texture(context.graph),
    );

    let normal_map_texture = texture_handle(
        context.doc,
        context.graph,
        context.load_context,
        m.normal_texture(context.graph),
    );

    let occlusion_texture = texture_handle(
        context.doc,
        context.graph,
        context.load_context,
        m.occlusion_texture(context.graph),
    );

    let mut material = StandardMaterial {
        alpha_mode,
        base_color: LinearRgba::from_f32_array(weight.base_color_factor).into(),
        base_color_texture,
        cull_mode,
        double_sided: weight.double_sided,
        emissive: LinearRgba::from_f32_array_no_alpha(weight.emissive_factor),
        emissive_texture,
        metallic: weight.metallic_factor,
        metallic_roughness_texture,
        normal_map_texture,
        occlusion_texture,
        perceptual_roughness: weight.roughness_factor,
        ..default()
    };

    E::import_material(context, &mut material, m);

    context
        .load_context
        .add_loaded_labeled_asset(label, LoadedAsset::new_with_dependencies(material))
}

const DEFAULT_MATERIAL_LABEL: &str = "MaterialDefault";

pub fn default_material(context: &mut ImportContext) -> Handle<StandardMaterial> {
    info!("Using default material");

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
    texture: Option<Texture>,
) -> Option<Handle<Image>> {
    let index = doc.texture_index(graph, texture?)?;
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
