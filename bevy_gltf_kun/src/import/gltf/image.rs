use bevy::{
    prelude::*,
    render::texture::{
        CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler,
        ImageSamplerDescriptor, ImageType,
    },
    utils::HashSet,
};
use gltf_kun::graph::{
    gltf::{
        document::GltfDocument,
        texture_info::{MagFilter, MinFilter, TextureInfoWeight, Wrap},
    },
    GraphNodeWeight,
};

use crate::import::extensions::BevyImportExtensions;

use super::document::ImportContext;

pub fn import_images<E: BevyImportExtensions<GltfDocument>>(context: &mut ImportContext) {
    let linear_images = context
        .doc
        .materials(context.graph)
        .iter()
        .flat_map(|m| {
            [
                m.metallic_roughness_texture_info(context.graph),
                m.normal_texture_info(context.graph),
                m.occlusion_texture_info(context.graph),
            ]
        })
        .flatten()
        .filter_map(|x| x.image(context.graph))
        .collect::<HashSet<_>>();

    context
        .doc
        .materials(context.graph)
        .iter()
        .flat_map(|m| {
            [
                m.base_color_texture_info(context.graph),
                m.emissive_texture_info(context.graph),
                m.metallic_roughness_texture_info(context.graph),
                m.normal_texture_info(context.graph),
                m.occlusion_texture_info(context.graph),
            ]
        })
        .flatten()
        .filter_map(|info| info.image(context.graph).map(|image| (info, image)))
        .for_each(|(info, image)| {
            let is_srgb = !linear_images.contains(&image);
            let info_weight = info.get(context.graph);
            let sampler_descriptor = image_sampler(info_weight);

            let image_weight = image.get(context.graph);
            let supported_compressed_formats = CompressedImageFormats::default();
            let mime_type = image_weight.mime_type.as_deref().unwrap_or_default();

            let image = Image::from_buffer(
                &image_weight.data,
                ImageType::MimeType(mime_type),
                supported_compressed_formats,
                is_srgb,
                ImageSampler::Descriptor(sampler_descriptor),
            );
        });
}

fn image_sampler(weight: &TextureInfoWeight) -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: address_mode(&weight.wrap_s),
        address_mode_v: address_mode(&weight.wrap_t),

        mag_filter: weight
            .mag_filter
            .map(|filter| match filter {
                MagFilter::Linear => ImageFilterMode::Linear,
                MagFilter::Nearest => ImageFilterMode::Nearest,
                MagFilter::Other(v) => {
                    warn!("Unsupported texture mag filter: {:?}", v);
                    ImageFilterMode::default()
                }
            })
            .unwrap_or_default(),

        min_filter: weight
            .min_filter
            .map(|filter| match filter {
                MinFilter::Linear
                | MinFilter::LinearMipmapLinear
                | MinFilter::LinearMipmapNearest => ImageFilterMode::Linear,
                MinFilter::Nearest
                | MinFilter::NearestMipmapLinear
                | MinFilter::NearestMipmapNearest => ImageFilterMode::Nearest,
                MinFilter::Other(v) => {
                    warn!("Unsupported texture min filter: {:?}", v);
                    ImageFilterMode::default()
                }
            })
            .unwrap_or_default(),

        ..default()
    }
}

fn address_mode(value: &Option<Wrap>) -> ImageAddressMode {
    match value {
        Some(value) => match value {
            Wrap::ClampToEdge => ImageAddressMode::ClampToEdge,
            Wrap::MirroredRepeat => ImageAddressMode::MirrorRepeat,
            Wrap::Repeat => ImageAddressMode::Repeat,
            Wrap::Other(v) => {
                warn!("Unsupported texture wrap mode: {:?}", v);
                ImageAddressMode::Repeat
            }
        },
        None => ImageAddressMode::Repeat,
    }
}
