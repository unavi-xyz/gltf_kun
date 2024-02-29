use bevy::{prelude::*, utils::HashMap};
use gltf_kun::graph::{
    gltf::{
        accessor::{ComponentType, Type},
        animation::{AnimationChannel, AnimationSampler, Interpolation, TargetPath},
        Animation, Node,
    },
    GraphNodeWeight,
};

use crate::import::gltf::animation::paths_recur;

use super::ExportContext;

pub fn export_animations(
    In(mut context): In<ExportContext>,
    clips: Res<Assets<AnimationClip>>,
) -> ExportContext {
    let mut animation_paths = HashMap::new();
    for scene in context.doc.scenes(&context.graph) {
        for node in scene.nodes(&context.graph) {
            paths_recur(
                &context.doc,
                &context.graph,
                &[],
                node,
                &mut animation_paths,
                node,
            );
        }
    }

    let nodes = context.nodes.iter().map(|n| n.node).collect::<Vec<_>>();

    // Find clips that have an EntityPath matching one of our nodes.
    for (_, clip) in clips.iter() {
        for node in &nodes {
            let parts = match animation_paths.get(node) {
                Some((_, path)) => path.clone(),
                None => {
                    warn!("No path found for node {:?}", node);
                    continue;
                }
            };

            let path = EntityPath { parts };

            let curves = match clip.get_curves_by_path(&path) {
                Some(curves) => curves,
                None => continue,
            };

            if curves.is_empty() {
                warn!("Curves empty for path {:?}", path);
                continue;
            }

            let a = context.doc.create_animation(&mut context.graph);

            for curve in curves.iter() {
                export_curve(&mut context, a, curve, *node);
            }
        }
    }

    context
}

fn export_curve(
    context: &mut ExportContext,
    animation: Animation,
    curve: &VariableCurve,
    target: Node,
) {
    let mut channel = AnimationChannel::new(&mut context.graph);
    animation.add_channel(&mut context.graph, &channel);

    let mut sampler = AnimationSampler::new(&mut context.graph);
    channel.set_sampler(&mut context.graph, Some(sampler));
    channel.set_target(&mut context.graph, Some(target));

    let sampler_weight = sampler.get_mut(&mut context.graph);

    let interpolation = match curve.interpolation {
        bevy::animation::Interpolation::Linear => Interpolation::Linear,
        bevy::animation::Interpolation::Step => Interpolation::Step,
        bevy::animation::Interpolation::CubicSpline => Interpolation::CubicSpline,
    };

    sampler_weight.interpolation = interpolation;

    let buffer = context.doc.create_buffer(&mut context.graph);

    let mut input = context.doc.create_accessor(&mut context.graph);
    input.set_buffer(&mut context.graph, Some(buffer));
    sampler.set_input(&mut context.graph, Some(input));

    let input_weight = input.get_mut(&mut context.graph);
    input_weight.element_type = Type::Scalar;
    input_weight.component_type = ComponentType::F32;
    input_weight.data = curve
        .keyframe_timestamps
        .iter()
        .copied()
        .flat_map(|f| f.to_le_bytes())
        .collect();

    let mut output = context.doc.create_accessor(&mut context.graph);
    output.set_buffer(&mut context.graph, Some(buffer));
    sampler.set_output(&mut context.graph, Some(output));

    match &curve.keyframes {
        Keyframes::Translation(keyframes) => {
            let channel_weight = channel.get_mut(&mut context.graph);
            channel_weight.path = TargetPath::Translation;

            let output_weight = output.get_mut(&mut context.graph);
            output_weight.element_type = Type::Vec3;
            output_weight.component_type = ComponentType::F32;
            output_weight.data = keyframes
                .iter()
                .flat_map(|v| v.to_array())
                .flat_map(|f| f.to_le_bytes())
                .collect();
        }
        Keyframes::Rotation(keyframes) => {
            let channel_weight = channel.get_mut(&mut context.graph);
            channel_weight.path = TargetPath::Rotation;

            let output_weight = output.get_mut(&mut context.graph);
            output_weight.element_type = Type::Vec4;
            output_weight.component_type = ComponentType::F32;
            output_weight.data = keyframes
                .iter()
                .flat_map(|v| v.to_array())
                .flat_map(|f| f.to_le_bytes())
                .collect();
        }
        Keyframes::Scale(keyframes) => {
            let channel_weight = channel.get_mut(&mut context.graph);
            channel_weight.path = TargetPath::Scale;

            let output_weight = output.get_mut(&mut context.graph);
            output_weight.element_type = Type::Vec3;
            output_weight.component_type = ComponentType::F32;
            output_weight.data = keyframes
                .iter()
                .flat_map(|v| v.to_array())
                .flat_map(|f| f.to_le_bytes())
                .collect();
        }
        Keyframes::Weights(keyframes) => {
            let channel_weight = channel.get_mut(&mut context.graph);
            channel_weight.path = TargetPath::MorphTargetWeights;

            let output_weight = output.get_mut(&mut context.graph);
            output_weight.element_type = Type::Scalar;
            output_weight.component_type = ComponentType::F32;
            output_weight.data = keyframes.iter().flat_map(|f| f.to_le_bytes()).collect();
        }
    }
}
