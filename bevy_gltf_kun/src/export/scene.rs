use bevy::prelude::*;
use gltf_kun::graph::gltf::scene;

use super::{CachedScene, ExportContext};

pub fn export_scenes(
    In(mut context): In<ExportContext>,
    names: Query<&Name>,
    scenes: Query<(Entity, &Handle<Scene>)>,
) -> ExportContext {
    for (entity, handle) in scenes.iter() {
        if !context.event.scenes.contains(handle) {
            continue;
        }

        if context.scenes.iter().any(|x| x.handle == *handle) {
            continue;
        }

        let mut scene = scene::Scene::new(&mut context.graph);

        if context.event.default_scene == Some(handle.clone()) {
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
