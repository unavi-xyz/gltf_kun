use bevy::{
    asset::RenderAssetUsages,
    image::{
        CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler,
        ImageSamplerDescriptor, ImageType,
    },
    platform::collections::HashSet,
    prelude::*,
};
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{
        Image as ImageKun, Texture,
        texture::{MagFilter, MinFilter, TextureWeight, WrappingMode},
    },
};
use thiserror::Error;

use super::document::ImportContext;

const DEFAULT_MIME: &str = "image/png";

#[must_use]
pub fn get_linear_textures(context: &ImportContext) -> HashSet<Texture> {
    let mut textures = HashSet::default();

    for m in context.doc.materials(context.graph) {
        if let Some(info) = m.metallic_roughness_texture(context.graph) {
            textures.insert(info);
        }
        if let Some(info) = m.normal_texture(context.graph) {
            textures.insert(info);
        }
        if let Some(info) = m.occlusion_texture(context.graph) {
            textures.insert(info);
        }
    }

    textures
}

#[derive(Debug, Error)]
pub enum TextureLoadError {
    #[error("failed to create texture: {0}")]
    Texture(#[from] TextureError),
}

pub fn load_texture(
    context: &mut ImportContext,
    info: Texture,
    image: ImageKun,
    is_srgb: bool,
) -> Result<Image, TextureLoadError> {
    let info_weight = info.get(context.graph);
    let sampler_descriptor = sampler_descriptor(info_weight);

    let image_weight = image.get(context.graph);
    let supported_compressed_formats = CompressedImageFormats::default();

    let image_type = image_weight.mime_type.as_deref().map_or_else(
        || {
            image_weight.uri.as_ref().map_or_else(
                || {
                    warn!(
                        "No mime type or uri found for image, defaulting to {}.",
                        DEFAULT_MIME
                    );
                    ImageType::MimeType(DEFAULT_MIME)
                },
                |uri| {
                    if uri.contains('.') {
                        uri.split('.').next_back().map_or_else(
                            || {
                                warn!(
                                    "No extension found for image uri, defaulting to {}.",
                                    DEFAULT_MIME
                                );
                                ImageType::MimeType(DEFAULT_MIME)
                            },
                            ImageType::Extension,
                        )
                    } else {
                        warn!(
                            "No extension found for image uri, defaulting to {}.",
                            DEFAULT_MIME
                        );
                        ImageType::MimeType(DEFAULT_MIME)
                    }
                },
            )
        },
        ImageType::MimeType,
    );

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

fn sampler_descriptor(weight: &TextureWeight) -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: address_mode(weight.wrap_s),
        address_mode_v: address_mode(weight.wrap_t),
        mag_filter: weight.mag_filter.map_or_else(
            || ImageSamplerDescriptor::default().mag_filter,
            |filter| match filter {
                MagFilter::Linear => ImageFilterMode::Linear,
                MagFilter::Nearest => ImageFilterMode::Nearest,
            },
        ),
        min_filter: weight.min_filter.map_or_else(
            || ImageSamplerDescriptor::default().min_filter,
            |filter| match filter {
                MinFilter::Linear
                | MinFilter::LinearMipmapLinear
                | MinFilter::LinearMipmapNearest => ImageFilterMode::Linear,
                MinFilter::Nearest
                | MinFilter::NearestMipmapLinear
                | MinFilter::NearestMipmapNearest => ImageFilterMode::Nearest,
            },
        ),
        mipmap_filter: weight.min_filter.map_or_else(
            || ImageSamplerDescriptor::default().mipmap_filter,
            |filter| match filter {
                MinFilter::LinearMipmapLinear | MinFilter::NearestMipmapLinear => {
                    ImageFilterMode::Linear
                }
                MinFilter::Linear
                | MinFilter::Nearest
                | MinFilter::LinearMipmapNearest
                | MinFilter::NearestMipmapNearest => ImageFilterMode::Nearest,
            },
        ),
        ..default()
    }
}

const fn address_mode(value: WrappingMode) -> ImageAddressMode {
    match value {
        WrappingMode::ClampToEdge => ImageAddressMode::ClampToEdge,
        WrappingMode::MirroredRepeat => ImageAddressMode::MirrorRepeat,
        WrappingMode::Repeat => ImageAddressMode::Repeat,
    }
}

#[must_use]
pub fn texture_label(index: usize) -> String {
    format!("Texture{index}")
}
