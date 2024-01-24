use bevy::prelude::*;
use gltf_kun::graph::GraphNodeWeight;

use super::{CachedScene, ExportContext};

pub fn export_scenes(
    In(mut context): In<ExportContext>,
    names: Query<&Name>,
    scenes: Query<(Entity, &Handle<Scene>)>,
) -> ExportContext {
    for (entity, handle) in scenes.iter() {
        if !context.target_scenes.contains(handle) {
            continue;
        }

        if context.scenes.iter().any(|x| x.handle == *handle) {
            continue;
        }

        let mut scene = context.doc.create_scene(&mut context.graph);

        if context.target_default_scene == Some(handle.clone()) {
            context
                .doc
                .set_default_scene(&mut context.graph, Some(&scene));
        }

        let weight = scene.get_mut(&mut context.graph);

        if let Ok(name) = names.get(entity) {
            weight.name = Some(name.to_string());
        }

        context.scenes.push(CachedScene {
            entity,
            handle: handle.clone(),
            scene,
        });
    }

    context
}
