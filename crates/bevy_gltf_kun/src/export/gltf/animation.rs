use anyhow::bail;
use bevy::{
    animation::{animated_field, AnimationEntityMut, AnimationTargetId},
    prelude::*,
    utils::HashMap,
};
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
    In(mut ctx): In<ExportContext>,
    clips: Res<Assets<AnimationClip>>,
) -> ExportContext {
    let mut animation_paths = HashMap::new();
    for scene in ctx.doc.scenes(&ctx.graph) {
        for node in scene.nodes(&ctx.graph) {
            paths_recur(&ctx.doc, &ctx.graph, &[], node, &mut animation_paths, node);
        }
    }

    let nodes = ctx.nodes.iter().map(|n| n.node).collect::<Vec<_>>();

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

            let curves = match clip.curves_for_target(AnimationTargetId::from_names(parts.iter())) {
                Some(curves) => curves,
                None => continue,
            };

            if curves.is_empty() {
                warn!("Curves empty for path {:?}", parts);
                continue;
            }

            let a = ctx.doc.create_animation(&mut ctx.graph);

            for curve in curves.iter() {
                if let Err(e) = export_curve(&mut ctx, a, curve, *node) {
                    warn!("Failed to export animation curve: {:?}", e);
                };
            }
        }
    }

    ctx
}

const POLL_RATE: f32 = 0.1;

fn export_curve(
    ctx: &mut ExportContext,
    animation: Animation,
    curve: &VariableCurve,
    target: Node,
) -> anyhow::Result<()> {
    let mut channel = AnimationChannel::new(&mut ctx.graph);
    animation.add_channel(&mut ctx.graph, &channel);

    let mut sampler = AnimationSampler::new(&mut ctx.graph);
    channel.set_sampler(&mut ctx.graph, Some(sampler));
    channel.set_target(&mut ctx.graph, Some(target));

    let buffer = ctx.doc.create_buffer(&mut ctx.graph);

    let mut input = ctx.doc.create_accessor(&mut ctx.graph);
    input.set_buffer(&mut ctx.graph, Some(buffer));
    sampler.set_input(&mut ctx.graph, Some(input));

    let mut output = ctx.doc.create_accessor(&mut ctx.graph);
    output.set_buffer(&mut ctx.graph, Some(buffer));
    sampler.set_output(&mut ctx.graph, Some(output));

    let path = match curve.0.evaluator_id() {
        EvaluatorId::ComponentField(t) => {
            let prop_t = animated_field!(Transform::translation);
            let prop_r = animated_field!(Transform::rotation);
            let prop_s = animated_field!(Transform::scale);

            let EvaluatorId::ComponentField(id_t) = prop_t.evaluator_id() else {
                unreachable!();
            };
            let EvaluatorId::ComponentField(id_r) = prop_r.evaluator_id() else {
                unreachable!();
            };
            let EvaluatorId::ComponentField(id_s) = prop_s.evaluator_id() else {
                unreachable!();
            };

            // TODO: Weights

            if t == id_t {
                let output_weight = output.get_mut(&mut ctx.graph);
                output_weight.element_type = Type::Vec3;
                output_weight.component_type = ComponentType::F32;

                TargetPath::Translation
            } else if t == id_r {
                let output_weight = output.get_mut(&mut ctx.graph);
                output_weight.element_type = Type::Vec4;
                output_weight.component_type = ComponentType::F32;

                TargetPath::Rotation
            } else if t == id_s {
                let output_weight = output.get_mut(&mut ctx.graph);
                output_weight.element_type = Type::Vec3;
                output_weight.component_type = ComponentType::F32;

                TargetPath::Scale
            } else {
                bail!("Custom animation targets not supported.");
            }
        }
        EvaluatorId::Type(_) => {
            bail!("Custom animation types not supported.");
        }
    };

    let channel_weight = channel.get_mut(&mut ctx.graph);
    channel_weight.path = path;

    let mut world = World::default();

    let ent = world.spawn(Transform::default()).id();
    let mut graph = AnimationGraph::default();
    let idx = graph.add_blend(1.0, graph.root);

    // Export every animation as a cubic spline, with an arbitrary polling rate.
    // We do not know the original keyframe timestamps, as that information is
    // lost when importing into Bevy's curve format (or at least, it is not
    // made public to outside crates).
    let sampler_weight = sampler.get_mut(&mut ctx.graph);
    sampler_weight.interpolation = Interpolation::CubicSpline;

    let mut eval = curve.0.create_evaluator();
    let interval = curve.0.domain();

    let mut t = interval.start();
    let mut keyframe_timestamps = Vec::new();
    let mut keyframes: Vec<u8> = Vec::new();

    loop {
        keyframe_timestamps.push(t);

        curve
            .0
            .apply(eval.as_mut(), t, 1.0, idx)
            .map_err(|e| anyhow::format_err!("{:?}", e))?;

        match path {
            TargetPath::Translation => {
                let tr = world.entity(ent).get::<Transform>().unwrap();
                keyframes.extend(
                    tr.translation
                        .to_array()
                        .into_iter()
                        .flat_map(|f| f.to_le_bytes()),
                );
            }
            TargetPath::Rotation => {
                let tr = world.entity(ent).get::<Transform>().unwrap();
                keyframes.extend(
                    tr.rotation
                        .to_array()
                        .into_iter()
                        .flat_map(|f| f.to_le_bytes()),
                );
            }
            TargetPath::Scale => {
                let tr = world.entity(ent).get::<Transform>().unwrap();
                keyframes.extend(
                    tr.scale
                        .to_array()
                        .into_iter()
                        .flat_map(|f| f.to_le_bytes()),
                );
            }
            TargetPath::MorphTargetWeights => {
                todo!();
            }
        };

        let ent_anim = world
            .query::<AnimationEntityMut>()
            .get_mut(&mut world, ent)
            .unwrap();

        eval.commit(ent_anim)
            .map_err(|e| anyhow::format_err!("{:?}", e))?;

        if t == interval.end() {
            break;
        }

        t += POLL_RATE;

        if t > interval.end() {
            t = interval.end();
        }
    }

    let input_weight = input.get_mut(&mut ctx.graph);
    input_weight.element_type = Type::Scalar;
    input_weight.component_type = ComponentType::F32;
    input_weight.data = keyframe_timestamps
        .into_iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();

    let output_weight = output.get_mut(&mut ctx.graph);
    output_weight.data = keyframes;

    Ok(())
}
