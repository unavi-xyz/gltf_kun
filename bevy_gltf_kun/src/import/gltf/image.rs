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
        texture_info::{MagFilter, MinFilter, TextureInfoWeight, WrappingMode},
        GltfDocument, Image as ImageKun, TextureInfo,
    },
    GraphNodeWeight,
};
use thiserror::Error;

use crate::import::extensions::BevyImportExtensions;

use super::document::ImportContext;

#[derive(Debug, Error)]
pub enum ImageImportError {
    #[error("Failed to load texture: {0}")]
    Texture(#[from] TextureError),
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

const DEFAULT_MIME: &str = "image/png";

pub fn load_texture(
    context: &mut ImportContext,
    info: TextureInfo,
    image: ImageKun,
    linear_images: &HashSet<ImageKun>,
) -> Result<Image, ImageImportError> {
    let is_srgb = !linear_images.contains(&image);
    let info_weight = info.get(context.graph);
    let sampler_descriptor = sampler_descriptor(info_weight);

    let image_weight = image.get(context.graph);
    let supported_compressed_formats = CompressedImageFormats::default();

    let image_type = match image_weight.mime_type.as_deref() {
        Some(mime_type) => ImageType::MimeType(mime_type),
        None => match &image_weight.uri {
            Some(uri) => match uri.split('.').last() {
                Some(ext) => ImageType::Extension(ext),
                None => {
                    warn!(
                        "No extension found for image uri, defaulting to {}.",
                        DEFAULT_MIME
                    );
                    ImageType::MimeType(DEFAULT_MIME)
                }
            },
            None => {
                warn!(
                    "No mime type or uri found for image, defaulting to {}.",
                    DEFAULT_MIME
                );
                ImageType::MimeType(DEFAULT_MIME)
            }
        },
    };

    let texture = Image::from_buffer(
        &image_weight.data,
        image_type,
        supported_compressed_formats,
        is_srgb,
        ImageSampler::Descriptor(sampler_descriptor),
        RenderAssetUsages::default(),
    )?;

    Ok(texture)
}

fn sampler_descriptor(weight: &TextureInfoWeight) -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: address_mode(&weight.wrap_s),
        address_mode_v: address_mode(&weight.wrap_t),
        mag_filter: weight
            .mag_filter
            .map(|filter| match filter {
                MagFilter::Linear => ImageFilterMode::Linear,
                MagFilter::Nearest => ImageFilterMode::Nearest,
            })
            .unwrap_or(ImageSamplerDescriptor::default().mag_filter),
        min_filter: weight
            .min_filter
            .map(|filter| match filter {
                MinFilter::Linear
                | MinFilter::LinearMipmapLinear
                | MinFilter::LinearMipmapNearest => ImageFilterMode::Linear,
                MinFilter::Nearest
                | MinFilter::NearestMipmapLinear
                | MinFilter::NearestMipmapNearest => ImageFilterMode::Nearest,
            })
            .unwrap_or(ImageSamplerDescriptor::default().min_filter),
        mipmap_filter: weight
            .min_filter
            .map(|filter| match filter {
                MinFilter::LinearMipmapLinear | MinFilter::NearestMipmapLinear => {
                    ImageFilterMode::Linear
                }
                MinFilter::Linear
                | MinFilter::Nearest
                | MinFilter::LinearMipmapNearest
                | MinFilter::NearestMipmapNearest => ImageFilterMode::Nearest,
            })
            .unwrap_or(ImageSamplerDescriptor::default().mipmap_filter),
        ..default()
    }
}

fn address_mode(value: &WrappingMode) -> ImageAddressMode {
    match value {
        WrappingMode::ClampToEdge => ImageAddressMode::ClampToEdge,
        WrappingMode::MirroredRepeat => ImageAddressMode::MirrorRepeat,
        WrappingMode::Repeat => ImageAddressMode::Repeat,
    }
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index)
}
