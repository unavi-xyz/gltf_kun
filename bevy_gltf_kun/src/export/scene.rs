use bevy::prelude::*;
use gltf_kun::graph::gltf::scene;

use super::{CachedScene, ExportContext};

pub fn export_scenes(
    In(mut context): In<ExportContext>,
    names: Query<&Name>,
    scenes: Query<Entity, With<Handle<Scene>>>,
) -> ExportContext {
    for entity in scenes.iter() {
        let mut scene = scene::Scene::new(&mut context.doc.0);
        let weight = scene.get_mut(&mut context.doc.0);

        if let Ok(name) = names.get(entity) {
            weight.name = Some(name.to_string());
        }

        context.scenes.push(CachedScene { scene, entity });
    }

    context
}
