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
        accessor::iter::{AccessorIter, AccessorIterCreateError},
        animation::{Interpolation, TargetPath},
    },
};
use thiserror::Error;

use super::{document::ImportContext, node::node_name};

#[derive(Debug, Error)]
pub enum AnimationImportError {
    #[error("Failed to create accessor iterator: {0}")]
    AccessorIter(#[from] AccessorIterCreateError),
}

pub fn import_animation(
    context: &mut ImportContext,
    paths: &HashMap<Node, (Node, Vec<Name>)>,
    animation: Animation,
) -> Result<(HashSet<Node>, Handle<AnimationClip>), AnimationImportError> {
    let mut clip = AnimationClip::default();
    let mut roots = HashSet::new();

    for channel in animation.channels(context.graph) {
        let channel_weight = channel.get(context.graph);

        let sampler = match channel.sampler(context.graph) {
            Some(sampler) => sampler,
            None => {
                debug!("Channel has no sampler");
                continue;
            }
        };

        let sampler_weight = sampler.get(context.graph);

        let input = match sampler.input(context.graph) {
            Some(input) => input,
            None => {
                debug!("Sampler has no input");
                continue;
            }
        };

        let input_iter = input.iter(context.graph)?;

        let keyframe_timestamps: Vec<f32> = match input_iter {
            AccessorIter::F32(iter) => iter.collect(),
            _ => {
                debug!("Input is not F32");
                continue;
            }
        };

        let output = match sampler.output(context.graph) {
            Some(output) => output,
            None => {
                debug!("Sampler has no output");
                continue;
            }
        };

        let output_iter = output.iter(context.graph)?;

        let maybe_curve =
            match &channel_weight.path {
                TargetPath::Translation => {
                    let translations: Vec<Vec3> = match output_iter {
                        AccessorIter::F32x3(iter) => iter.map(Vec3::from).collect(),
                        _ => {
                            debug!("Output is not F32x3");
                            continue;
                        }
                    };

                    let property = animated_field!(Transform::translation);

                    if keyframe_timestamps.len() == 1 {
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
                                keyframe_timestamps.into_iter().zip(translations),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::Step => SteppedKeyframeCurve::new(
                                keyframe_timestamps.into_iter().zip(translations),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::CubicSpline => {
                                CubicKeyframeCurve::new(keyframe_timestamps, translations)
                                    .ok()
                                    .map(|curve| {
                                        VariableCurve::new(AnimatableCurve::new(property, curve))
                                    })
                            }
                        }
                    }
                }
                TargetPath::Rotation => {
                    let rotations: Vec<Quat> = match output_iter {
                        AccessorIter::F32x4(iter) => iter.map(Quat::from_array).collect(),
                        _ => {
                            debug!("Output is not F32x4");
                            continue;
                        }
                    };

                    let property = animated_field!(Transform::rotation);

                    if keyframe_timestamps.len() == 1 {
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
                                keyframe_timestamps.into_iter().zip(rotations),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::Step => SteppedKeyframeCurve::new(
                                keyframe_timestamps.into_iter().zip(rotations),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::CubicSpline => CubicRotationCurve::new(
                                keyframe_timestamps,
                                rotations.into_iter().map(Vec4::from),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                        }
                    }
                }
                TargetPath::Scale => {
                    let scales: Vec<Vec3> = match output_iter {
                        AccessorIter::F32x3(iter) => iter.map(Vec3::from).collect(),
                        _ => {
                            debug!("Output is not F32x3");
                            continue;
                        }
                    };

                    let property = animated_field!(Transform::scale);

                    if keyframe_timestamps.len() == 1 {
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
                                keyframe_timestamps.into_iter().zip(scales),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::Step => SteppedKeyframeCurve::new(
                                keyframe_timestamps.into_iter().zip(scales),
                            )
                            .ok()
                            .map(|curve| VariableCurve::new(AnimatableCurve::new(property, curve))),
                            Interpolation::CubicSpline => {
                                CubicKeyframeCurve::new(keyframe_timestamps, scales)
                                    .ok()
                                    .map(|curve| {
                                        VariableCurve::new(AnimatableCurve::new(property, curve))
                                    })
                            }
                        }
                    }
                }
                TargetPath::MorphTargetWeights => {
                    let weights: Vec<f32> = match output_iter {
                        AccessorIter::F32(iter) => iter.collect(),
                        _ => {
                            debug!("Output is not F32");
                            continue;
                        }
                    };

                    if keyframe_timestamps.len() == 1 {
                        Some(VariableCurve::new(WeightsCurve(ConstantCurve::new(
                            Interval::EVERYWHERE,
                            weights,
                        ))))
                    } else {
                        match sampler_weight.interpolation {
                            Interpolation::Linear => {
                                WideLinearKeyframeCurve::new(keyframe_timestamps, weights)
                                    .ok()
                                    .map(WeightsCurve)
                                    .map(VariableCurve::new)
                            }
                            Interpolation::Step => {
                                WideSteppedKeyframeCurve::new(keyframe_timestamps, weights)
                                    .ok()
                                    .map(WeightsCurve)
                                    .map(VariableCurve::new)
                            }

                            Interpolation::CubicSpline => {
                                WideCubicKeyframeCurve::new(keyframe_timestamps, weights)
                                    .ok()
                                    .map(WeightsCurve)
                                    .map(VariableCurve::new)
                            }
                        }
                    }
                }
            };

        let Some(curve) = maybe_curve else {
            warn!("Invalid {:?} keyframe data.", channel_weight.path);
            continue;
        };

        let target = match channel.target(context.graph) {
            Some(target) => target,
            None => {
                debug!("Channel has no target");
                continue;
            }
        };

        let path = match paths.get(&target) {
            Some(path) => path.clone(),
            None => {
                debug!("Target has no path");
                continue;
            }
        };

        roots.insert(path.0);

        clip.add_variable_curve_to_target(AnimationTargetId::from_names(path.1.iter()), curve);
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
