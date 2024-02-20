use bevy::{asset::LoadContext, prelude::*};
use gltf_kun::graph::{
    gltf::{
        document::GltfDocument,
        image,
        material::{Material, MaterialWeight},
    },
    Graph, GraphNodeWeight,
};
use thiserror::Error;

use crate::import::{extensions::BevyImportExtensions, util::asset_label};

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
    let label = material_label(index, weight);

    let handle = context
        .load_context
        .labeled_asset_scope(label, |load_context| {
            let base_color_texture = m.base_color_texture_info(context.graph);

            let base_color_texture = base_color_texture
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
    image: image::Image,
) -> Handle<Image> {
    let image_index = doc.images(graph).iter().position(|x| *x == image).unwrap();
    let label = texture_label(image_index);
    load_context.get_label_handle(&label)
}

fn texture_label(index: usize) -> String {
    asset_label("Texture", index, None)
}

fn material_label(index: usize, weight: &MaterialWeight) -> String {
    asset_label("Material", index, weight.name.as_deref())
}
