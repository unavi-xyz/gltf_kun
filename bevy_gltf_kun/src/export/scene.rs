use std::collections::BTreeMap;

use bevy::prelude::*;
use gltf_kun::graph::gltf::scene;

use super::ExportContext;

pub type SceneNodes = BTreeMap<scene::Scene, Vec<Entity>>;

pub fn export_scenes(
    In(mut context): In<ExportContext>,
    names: Query<&Name>,
    scenes: Query<(Entity, Option<&Children>), With<Handle<Scene>>>,
) -> (ExportContext, SceneNodes) {
    let mut scene_nodes = SceneNodes::new();

    for (entity, children) in scenes.iter() {
        let mut scene = scene::Scene::new(&mut context.doc.0);
        let weight = scene.get_mut(&mut context.doc.0);

        if let Ok(name) = names.get(entity) {
            weight.name = Some(name.to_string());
        }

        if let Some(children) = children {
            scene_nodes.insert(scene, children.to_vec());
        }
    }

    (context, scene_nodes)
}
