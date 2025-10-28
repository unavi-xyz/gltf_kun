use anyhow::{Context, Result, bail};
use bevy::{
    animation::{
        AnimationTargetId, animated_field,
        gltf_curves::{
            CubicKeyframeCurve, CubicRotationCurve, SteppedKeyframeCurve, WideCubicKeyframeCurve,
            WideLinearKeyframeCurve, WideSteppedKeyframeCurve,
        },
    },
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use gltf_kun::graph::{
    Graph, GraphNodeWeight,
    gltf::{
        Animation, GltfDocument, Node,
        accessor::iter::AccessorIter,
        animation::{Interpolation, TargetPath},
    },
};

use super::{document::ImportContext, node::node_name};

/// Raw animation curve data before conversion to Bevy's AnimatableCurve.
/// Useful for animation retargeting and custom processing.
#[derive(Asset, Debug, TypePath)]
pub struct RawGltfAnimation {
    pub name: Option<String>,
    pub channels: Vec<RawAnimationChannel>,
}

/// A single animation channel targeting a specific property of a node.
#[derive(Debug, Clone)]
pub struct RawAnimationChannel {
    /// Path from root to the target node
    pub target_path: Vec<Name>,
    /// Root node of the animation
    pub target_root: Node,
    /// The animated property and its curve data
    pub data: RawChannelData,
    /// Interpolation method
    pub interpolation: Interpolation,
}

/// Raw animation curve data for different property types.
#[derive(Debug, Clone)]
pub enum RawChannelData {
    Translation {
        timestamps: Vec<f32>,
        values: Vec<Vec3>,
    },
    Rotation {
        timestamps: Vec<f32>,
        values: Vec<Quat>,
    },
    Scale {
        timestamps: Vec<f32>,
        values: Vec<Vec3>,
    },
    MorphTargetWeights {
        timestamps: Vec<f32>,
        values: Vec<f32>,
    },
}

pub fn import_animation(
    context: &mut ImportContext,
    paths: &HashMap<Node, (Node, Vec<Name>)>,
    animation: Animation,
) -> Result<(HashSet<Node>, Handle<AnimationClip>)> {
    let mut clip = AnimationClip::default();
    let mut roots = HashSet::new();
    let mut raw_channels = Vec::new();

    for channel in animation.channels(context.graph) {
        let channel_weight = channel.get(context.graph);

        let sampler = channel
            .sampler(context.graph)
            .context("Channel has no sampler")?;

        let sampler_weight = sampler.get(context.graph);

        let input = sampler
            .input(context.graph)
            .context("Sampler has no input")?;

        let input_iter = input.iter(context.graph)?;

        let keyframe_timestamps: Vec<f32> = match input_iter {
            AccessorIter::F32(iter) => iter.collect(),
            _ => bail!("Input accessor is not F32"),
        };

        let output = sampler
            .output(context.graph)
            .context("Sampler has no output")?;

        let output_iter = output.iter(context.graph)?;

        let (maybe_curve, raw_data) = match &channel_weight.path {
            TargetPath::Translation => {
                let translations: Vec<Vec3> = match output_iter {
                    AccessorIter::F32x3(iter) => iter.map(Vec3::from).collect(),
                    _ => bail!("Translation output accessor is not F32x3"),
                };

                let raw_data = RawChannelData::Translation {
                    timestamps: keyframe_timestamps.clone(),
                    values: translations.clone(),
                };

                let property = animated_field!(Transform::translation);

                let curve = if keyframe_timestamps.len() == 1 {
                    if let Some(translation) = translations.first() {
                        Some(VariableCurve::new(AnimatableCurve::new(
                            property,
                            ConstantCurve::new(Interval::EVERYWHERE, *translation),
                        )))
                    } else {
                        warn!("No translation data.");
                        continue;
                    }
                } else {
                    match sampler_weight.interpolation {
                        Interpolation::Linear => UnevenSampleAutoCurve::new(
                            keyframe_timestamps
                                .clone()
                                .into_iter()
                                .zip(translations.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::Step => SteppedKeyframeCurve::new(
                            keyframe_timestamps
                                .clone()
                                .into_iter()
                                .zip(translations.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::CubicSpline => CubicKeyframeCurve::new(
                            keyframe_timestamps.clone(),
                            translations.clone(),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                    }
                };

                (curve, raw_data)
            }
            TargetPath::Rotation => {
                let rotations: Vec<Quat> = match output_iter {
                    AccessorIter::F32x4(iter) => iter.map(Quat::from_array).collect(),
                    _ => bail!("Rotation output accessor is not F32x4"),
                };

                let raw_data = RawChannelData::Rotation {
                    timestamps: keyframe_timestamps.clone(),
                    values: rotations.clone(),
                };

                let property = animated_field!(Transform::rotation);

                let curve = if keyframe_timestamps.len() == 1 {
                    if let Some(rotation) = rotations.first() {
                        Some(VariableCurve::new(AnimatableCurve::new(
                            property,
                            ConstantCurve::new(Interval::EVERYWHERE, *rotation),
                        )))
                    } else {
                        warn!("No rotation data.");
                        continue;
                    }
                } else {
                    match sampler_weight.interpolation {
                        Interpolation::Linear => UnevenSampleAutoCurve::new(
                            keyframe_timestamps
                                .clone()
                                .into_iter()
                                .zip(rotations.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::Step => SteppedKeyframeCurve::new(
                            keyframe_timestamps
                                .clone()
                                .into_iter()
                                .zip(rotations.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::CubicSpline => CubicRotationCurve::new(
                            keyframe_timestamps.clone(),
                            rotations.clone().into_iter().map(Vec4::from),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                    }
                };

                (curve, raw_data)
            }
            TargetPath::Scale => {
                let scales: Vec<Vec3> = match output_iter {
                    AccessorIter::F32x3(iter) => iter.map(Vec3::from).collect(),
                    _ => bail!("Scale output accessor is not F32x3"),
                };

                let raw_data = RawChannelData::Scale {
                    timestamps: keyframe_timestamps.clone(),
                    values: scales.clone(),
                };

                let property = animated_field!(Transform::scale);

                let curve = if keyframe_timestamps.len() == 1 {
                    if let Some(scale) = scales.first() {
                        Some(VariableCurve::new(AnimatableCurve::new(
                            property,
                            ConstantCurve::new(Interval::EVERYWHERE, *scale),
                        )))
                    } else {
                        warn!("No scale data.");
                        continue;
                    }
                } else {
                    match sampler_weight.interpolation {
                        Interpolation::Linear => UnevenSampleAutoCurve::new(
                            keyframe_timestamps.clone().into_iter().zip(scales.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::Step => SteppedKeyframeCurve::new(
                            keyframe_timestamps.clone().into_iter().zip(scales.clone()),
                        )
                        .ok()
                        .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        Interpolation::CubicSpline => {
                            CubicKeyframeCurve::new(keyframe_timestamps.clone(), scales.clone())
                                .ok()
                                .map(|curve| {
                                    VariableCurve::new(AnimatableCurve::new(property, curve))
                                })
                        }
                    }
                };

                (curve, raw_data)
            }
            TargetPath::MorphTargetWeights => {
                let weights: Vec<f32> = match output_iter {
                    AccessorIter::F32(iter) => iter.collect(),
                    _ => bail!("MorphTargetWeights output accessor is not F32"),
                };

                let raw_data = RawChannelData::MorphTargetWeights {
                    timestamps: keyframe_timestamps.clone(),
                    values: weights.clone(),
                };

                let curve = if keyframe_timestamps.len() == 1 {
                    Some(VariableCurve::new(WeightsCurve(ConstantCurve::new(
                        Interval::EVERYWHERE,
                        weights.clone(),
                    ))))
                } else {
                    match sampler_weight.interpolation {
                        Interpolation::Linear => WideLinearKeyframeCurve::new(
                            keyframe_timestamps.clone(),
                            weights.clone(),
                        )
                        .ok()
                        .map(WeightsCurve)
                        .map(VariableCurve::new),
                        Interpolation::Step => WideSteppedKeyframeCurve::new(
                            keyframe_timestamps.clone(),
                            weights.clone(),
                        )
                        .ok()
                        .map(WeightsCurve)
                        .map(VariableCurve::new),

                        Interpolation::CubicSpline => WideCubicKeyframeCurve::new(
                            keyframe_timestamps.clone(),
                            weights.clone(),
                        )
                        .ok()
                        .map(WeightsCurve)
                        .map(VariableCurve::new),
                    }
                };

                (curve, raw_data)
            }
        };

        let Some(curve) = maybe_curve else {
            warn!("Invalid {:?} keyframe data.", channel_weight.path);
            continue;
        };

        let target = channel
            .target(context.graph)
            .context("Channel has no target")?;

        let path = paths.get(&target).cloned().context("Target has no path")?;

        roots.insert(path.0);

        clip.add_variable_curve_to_target(AnimationTargetId::from_names(path.1.iter()), curve);

        // Store raw channel data if requested
        if context.expose_raw_curves {
            raw_channels.push(RawAnimationChannel {
                target_path: path.1.clone(),
                target_root: path.0,
                data: raw_data,
                interpolation: sampler_weight.interpolation,
            });
        }
    }

    let index = context
        .doc
        .animation_index(context.graph, animation)
        .unwrap();
    let animation_label = format!("Animation{}", index);

    let handle = context
        .load_context
        .add_labeled_asset(animation_label, clip);

    let weight = animation.get(context.graph);
    if let Some(name) = &weight.name {
        context
            .gltf
            .named_animations
            .insert(name.to_string(), handle.clone());
    }

    // Create raw animation asset if requested
    if context.expose_raw_curves {
        let raw_animation = RawGltfAnimation {
            name: weight.name.clone(),
            channels: raw_channels,
        };

        let raw_label = format!("RawAnimation{}", index);
        let raw_handle = context
            .load_context
            .add_labeled_asset(raw_label, raw_animation);

        context.gltf.raw_animations.push(raw_handle.clone());

        if let Some(name) = &weight.name {
            context
                .gltf
                .named_raw_animations
                .insert(name.to_string(), raw_handle);
        }
    }

    Ok((roots, handle))
}

/// Get the path of names from the root to the given node.
pub fn paths_recur(
    doc: &GltfDocument,
    graph: &Graph,
    current_path: &[Name],
    node: Node,
    paths: &mut HashMap<Node, (Node, Vec<Name>)>,
    root: Node,
) {
    let mut path = current_path.to_owned();

    let name = node_name(doc, graph, node);
    let name = Name::new(name);

    path.push(name);

    for child in node.children(graph) {
        paths_recur(doc, graph, &path, child, paths, root);
    }

    paths.insert(node, (root, path));
}
