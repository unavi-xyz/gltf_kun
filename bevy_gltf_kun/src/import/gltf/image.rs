use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        texture::{
            CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler,
            ImageSamplerDescriptor, ImageType, TextureError,
        },
    },
    utils::HashSet,
};
use gltf_kun::graph::{
    gltf::{
        document::GltfDocument,
        texture_info::{MagFilter, MinFilter, TextureInfoWeight, Wrap},
        Image as ImageKun, TextureInfo,
    },
    GraphNodeWeight,
};
use thiserror::Error;

use crate::import::{extensions::BevyImportExtensions, util::asset_label};

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum ImageImportError {
    #[error("Failed to load texture: {0}")]
    Texture(#[from] TextureError),
    #[error("Missing mime type")]
    MissingMimeType,
}

pub fn import_images<E: BevyImportExtensions<GltfDocument>>(
    context: &mut ImportContext,
) -> Result<(), ImageImportError> {
    let mut linear_images = HashSet::default();
    let mut texture_infos = HashSet::default();

    for m in context.doc.materials(context.graph) {
        if let Some(info) = m.base_color_texture_info(context.graph) {
            texture_infos.insert(info);
        }

        if let Some(info) = m.emissive_texture_info(context.graph) {
            texture_infos.insert(info);
        }

        if let Some(info) = m.metallic_roughness_texture_info(context.graph) {
            texture_infos.insert(info);

            if let Some(image) = info.image(context.graph) {
                linear_images.insert(image);
            }
        }

        if let Some(info) = m.normal_texture_info(context.graph) {
            texture_infos.insert(info);

            if let Some(image) = info.image(context.graph) {
                linear_images.insert(image);
            }
        }

        if let Some(info) = m.occlusion_texture_info(context.graph) {
            texture_infos.insert(info);

            if let Some(image) = info.image(context.graph) {
                linear_images.insert(image);
            }
        }
    }

    for (i, info) in texture_infos.iter().enumerate() {
        if let Some(image) = info.image(context.graph) {
            let label = texture_label(i);
            let texture = load_texture(context, *info, image, &linear_images)?;
            let handle = context.load_context.add_labeled_asset(label, texture);
            context.gltf.images.insert(i, handle);
        }
    }

    Ok(())
}

pub fn load_texture(
    context: &mut ImportContext,
    info: TextureInfo,
    image: ImageKun,
    linear_images: &HashSet<ImageKun>,
) -> Result<Image, ImageImportError> {
    let is_srgb = !linear_images.contains(&image);
    let info_weight = info.get(context.graph);
    let sampler_descriptor = image_sampler(info_weight);

    let image_weight = image.get(context.graph);
    let supported_compressed_formats = CompressedImageFormats::default();
    let mime_type = image_weight
        .mime_type
        .as_deref()
        .ok_or(ImageImportError::MissingMimeType)?;

    let texture = Image::from_buffer(
        &image_weight.data,
        ImageType::MimeType(mime_type),
        supported_compressed_formats,
        is_srgb,
        ImageSampler::Descriptor(sampler_descriptor),
        RenderAssetUsages::default(),
    )?;

    Ok(texture)
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

fn texture_label(index: usize) -> String {
    asset_label("Texture", index, None)
}
