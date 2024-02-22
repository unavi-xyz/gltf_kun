use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::graph::{
    gltf::{self, GltfDocument, Material},
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
            let base_color_texture = m
                .base_color_texture_info(context.graph)
                .and_then(|info| info.image(context.graph))
                .map(|image| texture_handle(context.doc, context.graph, load_context, image));

            StandardMaterial {
                base_color: Color::rgba_linear(
                    weight.base_color_factor[0],
                    weight.base_color_factor[1],
                    weight.base_color_factor[2],
                    weight.base_color_factor[3],
                ),
                base_color_texture,
                ..default()
            }
        });

    Ok(handle)
}

fn texture_handle(
    doc: &mut GltfDocument,
    graph: &Graph,
    load_context: &mut LoadContext,
    image: gltf::image::Image,
) -> Handle<Image> {
    let image_index = doc.images(graph).iter().position(|x| *x == image).unwrap();
    let label = texture_label(image_index);
    load_context.get_label_handle(&label)
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index,)
}

fn material_label(index: usize) -> String {
    format!("Material{}", index)
}
