use bevy::prelude::*;
use gltf_kun::graph::GraphNodeWeight;

use super::{CachedScene, ExportContext};

pub fn export_scenes(
    In(mut ctx): In<ExportContext>,
    names: Query<&Name>,
    scenes: Query<(Entity, &SceneRoot)>,
) -> ExportContext {
    for (entity, handle) in scenes.iter() {
        if !ctx.target_scenes.contains(handle) {
            continue;
        }

        if ctx.scenes.iter().any(|x| x.handle == **handle) {
            continue;
        }

        let mut scene = ctx.doc.create_scene(&mut ctx.graph);

        if ctx.target_default_scene == Some(handle.0.clone()) {
            ctx.doc.set_default_scene(&mut ctx.graph, Some(scene));
        }

        let weight = scene.get_mut(&mut ctx.graph);

        if let Ok(name) = names.get(entity) {
            weight.name = Some(name.to_string());
        }

        ctx.scenes.push(CachedScene {
            entity,
            handle: handle.0.clone(),
            scene,
        });
    }

    ctx
}
