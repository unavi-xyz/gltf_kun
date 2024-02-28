use bevy::{prelude::*, utils::HashMap};
use gltf_kun::graph::{
    gltf::{
        accessor::{iter::AccessorIter, GetAccessorIterError},
        animation::{Interpolation, TargetPath},
        Animation, Node,
    },
    GraphNodeWeight,
};
use thiserror::Error;

use super::{document::ImportContext, node::node_name};

#[derive(Debug, Error)]
pub enum AnimationImportError {
    #[error("Failed to get accessor iterator: {0}")]
    GetAccessorIter(#[from] GetAccessorIterError),
}

pub fn import_animation(
    context: &mut ImportContext,
    paths: &HashMap<Node, (Node, Vec<Name>)>,
    animation: Animation,
) -> Result<Handle<AnimationClip>, AnimationImportError> {
    let mut clip = AnimationClip::default();

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

        let interpolation = match sampler_weight.interpolation {
            Interpolation::CubicSpline => bevy::animation::Interpolation::CubicSpline,
            Interpolation::Linear => bevy::animation::Interpolation::Linear,
            Interpolation::Step => bevy::animation::Interpolation::Step,
        };

        let input = match sampler.input(context.graph) {
            Some(input) => input,
            None => {
                debug!("Sampler has no input");
                continue;
            }
        };

        let input_iter = input.iter(context.graph)?;

        let keyframe_timestamps = match input_iter {
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

        let keyframes = match &channel_weight.path {
            TargetPath::Translation => {
                let iter = match output_iter {
                    AccessorIter::F32x3(iter) => iter,
                    _ => {
                        debug!("Output is not F32x3");
                        continue;
                    }
                };

                Keyframes::Translation(iter.map(Vec3::from).collect())
            }
            TargetPath::Rotation => {
                let iter = match output_iter {
                    AccessorIter::F32x4(iter) => iter,
                    _ => {
                        debug!("Output is not F32x4");
                        continue;
                    }
                };

                Keyframes::Rotation(iter.map(Quat::from_array).collect())
            }
            TargetPath::Scale => {
                let iter = match output_iter {
                    AccessorIter::F32x3(iter) => iter,
                    _ => {
                        debug!("Output is not F32x3");
                        continue;
                    }
                };

                Keyframes::Scale(iter.map(Vec3::from).collect())
            }
            TargetPath::MorphTargetWeights => {
                let iter = match output_iter {
                    AccessorIter::F32(iter) => iter,
                    _ => {
                        debug!("Output is not F32");
                        continue;
                    }
                };

                Keyframes::Weights(iter.collect())
            }
        };

        let target = match channel.target(context.graph) {
            Some(target) => target,
            None => {
                debug!("Channel has no target");
                continue;
            }
        };

        let path = match paths.get(&target) {
            Some(path) => path,
            None => {
                debug!("Target has no path");
                continue;
            }
        };

        clip.add_curve_to_path(
            EntityPath {
                parts: path.1.clone(),
            },
            VariableCurve {
                interpolation,
                keyframe_timestamps,
                keyframes,
            },
        );
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

    Ok(handle)
}

/// Get the path of names from the root to the given node.
pub fn paths_recur(
    context: &ImportContext,
    mut current_path: Vec<Name>,
    node: Node,
    paths: &mut HashMap<Node, (Node, Vec<Name>)>,
    root: Node,
) {
    let name = node_name(context, node);
    let name = Name::new(name);

    current_path.push(name);
    paths.insert(node, (root, current_path.clone()));

    for child in node.children(context.graph) {
        paths_recur(context, current_path.clone(), child, paths, root);
    }
}
