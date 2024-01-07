use anyhow::Result;
use bevy::prelude::*;
use gltf_kun::{
    document::gltf::GltfDocument,
    graph::gltf::{buffer, buffer_view, mesh, node, primitive, scene},
    io::format::ImportFormat,
};

use crate::{BevyFormat, Export, ExportResult};

impl ImportFormat<GltfDocument> for BevyFormat {
    fn import(self) -> Result<GltfDocument> {
        todo!()
    }
}

pub fn export_gltf(
    mut reader: EventReader<Export>,
    mut writer: EventWriter<ExportResult>,
    scenes: Query<(Entity, &Handle<Scene>)>,
    nodes: Query<&Transform>,
    names: Query<&Name>,
) {
    for event in reader.read() {
        let mut doc = GltfDocument::default();

        // Create scenes
        event
            .scenes
            .iter()
            .filter_map(|entity| match scenes.get(*entity) {
                Ok(scene) => Some(scene),
                Err(_) => {
                    warn!("Scene not found: {:?}", entity);
                    None
                }
            })
            .for_each(|(entity, s)| {
                let mut scene = scene::Scene::new(&mut doc.0);
                let weight = scene.get_mut(&mut doc.0);

                if let Ok(name) = names.get(entity) {
                    weight.name = Some(name.to_string());
                }
            });

        // TODO: Create nodes

        writer.send(ExportResult {
            result: Ok(Box::new(doc)),
        });
    }
}
