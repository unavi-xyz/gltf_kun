use std::marker::PhantomData;

use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageSampler},
    prelude::*,
    render::render_resource::TextureFormat,
};
use gltf_kun::graph::{
    GraphNodeWeight,
    gltf::{
        Texture,
        accessor::iter::ElementIter,
        material::{AlphaCutoff, AlphaMode},
        texture::{MagFilter, MinFilter, WrappingMode},
    },
};
use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};
use thiserror::Error;

use super::{CachedMaterial, ExportContext};

pub fn export_materials(
    In(mut ctx): In<ExportContext>,
    material_assets: Res<Assets<StandardMaterial>>,
    materials: Query<(&MeshMaterial3d<StandardMaterial>, Option<&Name>)>,
    image_assets: Res<Assets<Image>>,
) -> ExportContext {
    for mesh in ctx.doc.meshes(&ctx.graph) {
        let cached_mesh = ctx
            .meshes
            .iter()
            .find(|cached| cached.mesh == mesh)
            .unwrap();

        for (entity, primitive) in cached_mesh.primitives.clone() {
            let (handle, name) = match materials.get(entity) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let cached_material = ctx
                .materials
                .iter()
                .find(|cached| cached.bevy_material.0 == handle.0);

            let material = match cached_material {
                Some(cached) => cached.material,
                None => {
                    let standard_material = material_assets.get(handle).unwrap();

                    let mut material = ctx.doc.create_material(&mut ctx.graph);
                    let weight = material.get_mut(&mut ctx.graph);

                    weight.name = name.map(|n| n.to_string());
                    weight.double_sided = standard_material.double_sided;
                    weight.metallic_factor = standard_material.metallic;
                    weight.roughness_factor = standard_material.perceptual_roughness;
                    weight.base_color_factor =
                        standard_material.base_color.to_linear().to_f32_array();
                    weight.emissive_factor = standard_material.emissive.to_f32_array_no_alpha();

                    let alpha_mode = match standard_material.alpha_mode {
                        bevy::prelude::AlphaMode::Blend => AlphaMode::Blend,
                        bevy::prelude::AlphaMode::Mask(cutoff) => {
                            weight.alpha_cutoff = AlphaCutoff(cutoff);
                            AlphaMode::Mask
                        }
                        bevy::prelude::AlphaMode::Opaque => AlphaMode::Opaque,
                        _ => {
                            warn!("Unsupported alpha mode: {:?}", standard_material.alpha_mode);
                            AlphaMode::Opaque
                        }
                    };
                    weight.alpha_mode = alpha_mode;

                    let base_color_texture = export_texture(
                        &mut ctx,
                        &standard_material.base_color_texture,
                        &image_assets,
                    );
                    material.set_base_color_texture(&mut ctx.graph, base_color_texture);

                    let emissive_texture = export_texture(
                        &mut ctx,
                        &standard_material.emissive_texture,
                        &image_assets,
                    );
                    material.set_emissive_texture(&mut ctx.graph, emissive_texture);

                    let metallic_roughness_texture = export_texture(
                        &mut ctx,
                        &standard_material.metallic_roughness_texture,
                        &image_assets,
                    );
                    material
                        .set_metallic_roughness_texture(&mut ctx.graph, metallic_roughness_texture);

                    let normal_texture = export_texture(
                        &mut ctx,
                        &standard_material.normal_map_texture,
                        &image_assets,
                    );
                    material.set_normal_texture(&mut ctx.graph, normal_texture);

                    let occlusion_texture = export_texture(
                        &mut ctx,
                        &standard_material.occlusion_texture,
                        &image_assets,
                    );
                    material.set_occlusion_texture(&mut ctx.graph, occlusion_texture);

                    ctx.materials.push(CachedMaterial {
                        bevy_material: handle.clone(),
                        entity,
                        material,
                    });

                    material
                }
            };

            primitive.set_material(&mut ctx.graph, Some(material));
        }
    }

    ctx
}

fn export_texture(
    ctx: &mut ExportContext,
    texture: &Option<Handle<Image>>,
    image_assets: &Res<Assets<Image>>,
) -> Option<Texture> {
    let handle = match texture {
        Some(handle) => handle,
        None => return None,
    };

    let bevy_image = image_assets.get(handle).unwrap();

    let mut image = ctx.doc.create_image(&mut ctx.graph);

    let buffer = ctx.doc.buffers(&ctx.graph)[0];
    image.set_buffer(&mut ctx.graph, Some(buffer));

    let image_weight = image.get_mut(&mut ctx.graph);

    let (mime, data) = match convert_image(bevy_image) {
        Ok((mime, data)) => (mime, data),
        Err(e) => {
            warn!("Failed to convert image to glTF supported format: {:?}", e);
            return None;
        }
    };

    image_weight.mime_type = Some(mime);
    image_weight.data = data;

    let mut texture = ctx.doc.create_texture(&mut ctx.graph);
    texture.set_image(&mut ctx.graph, Some(image));

    let info_weight = texture.get_mut(&mut ctx.graph);

    match &bevy_image.sampler {
        ImageSampler::Default => {
            info_weight.wrap_s = WrappingMode::ClampToEdge;
            info_weight.wrap_t = WrappingMode::ClampToEdge;
        }
        ImageSampler::Descriptor(desc) => {
            info_weight.wrap_s = address_mode(&desc.address_mode_u);
            info_weight.wrap_t = address_mode(&desc.address_mode_v);

            info_weight.mag_filter = Some(match desc.mag_filter {
                ImageFilterMode::Linear => MagFilter::Linear,
                ImageFilterMode::Nearest => MagFilter::Nearest,
            });

            info_weight.min_filter = Some(match desc.min_filter {
                ImageFilterMode::Linear => match desc.mipmap_filter {
                    ImageFilterMode::Linear => MinFilter::LinearMipmapLinear,
                    ImageFilterMode::Nearest => MinFilter::LinearMipmapNearest,
                },
                ImageFilterMode::Nearest => match desc.mipmap_filter {
                    ImageFilterMode::Linear => MinFilter::NearestMipmapLinear,
                    ImageFilterMode::Nearest => MinFilter::NearestMipmapNearest,
                },
            });
        }
    };

    Some(texture)
}

fn address_mode(value: &ImageAddressMode) -> WrappingMode {
    match value {
        ImageAddressMode::ClampToBorder | ImageAddressMode::ClampToEdge => {
            WrappingMode::ClampToEdge
        }
        ImageAddressMode::MirrorRepeat => WrappingMode::MirroredRepeat,
        ImageAddressMode::Repeat => WrappingMode::Repeat,
    }
}

#[derive(Debug, Error)]
pub enum ConvertImageError {
    #[error("failed to convert image")]
    FailedToConvert,
    #[error("unsupported format")]
    UnsupportedFormat,
}

// Converts a Bevy texture to a glTF supported format.
// Returns the mime type and new image data
fn convert_image(bevy_image: &Image) -> Result<(String, Vec<u8>), ConvertImageError> {
    let desc = &bevy_image.texture_descriptor;

    match desc.format {
        TextureFormat::Rgba8Sint | TextureFormat::Rgba8Snorm => {
            let data = bevy_image.data.clone().unwrap_or_default();
            let iter = ElementIter::<i8> {
                slice: &data,
                normalized: true,
                _phantom: PhantomData,
            };
            let data = iter.map(|v| (v + 127) as u8).collect::<Vec<_>>();
            convert_png(
                &data,
                desc.size.width,
                desc.size.height,
                ExtendedColorType::Rgba8,
            )
        }
        TextureFormat::Rgba8Uint | TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            convert_png(
                bevy_image.data.clone().unwrap_or_default().as_slice(),
                desc.size.width,
                desc.size.height,
                ExtendedColorType::Rgba8,
            )
        }

        _ => {
            warn!("Unsupported texture format: {:?}", desc.format);
            Err(ConvertImageError::UnsupportedFormat)
        }
    }
}

fn convert_png(
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ExtendedColorType,
) -> Result<(String, Vec<u8>), ConvertImageError> {
    let mut out = Vec::new();
    let encoder = PngEncoder::new(&mut out);
    encoder
        .write_image(data, width, height, color_type)
        .map_err(|_| ConvertImageError::FailedToConvert)?;
    Ok(("image/png".to_string(), out))
}
