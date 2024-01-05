use anyhow::Result;
use glam::Quat;

use crate::{document::Document, graph::node::Node};

use super::IoFormat;

pub struct GltfFormat {
    pub json: gltf::json::Root,
    pub blob: Option<Vec<u8>>,
}

impl IoFormat for GltfFormat {
    fn import(mut self) -> Result<Document> {
        let mut doc = Document::default();

        // TODO: Create accessors
        // TODO: Create materials
        // TODO: Create meshes

        // Create nodes
        let nodes = self
            .json
            .nodes
            .iter_mut()
            .map(|n| {
                let mut node = Node::new(&mut doc.0);
                let weight = node.get_mut(&mut doc.0);

                weight.name = n.name.take();
                weight.extras = n.extras.take();

                weight.translation = n.translation.map(|t| t.into()).unwrap_or_default();
                weight.rotation = n
                    .rotation
                    .map(|r| Quat::from_slice(&r.0))
                    .unwrap_or(Quat::IDENTITY);
                weight.scale = n.scale.map(|s| s.into()).unwrap_or(glam::Vec3::ONE);

                node
            })
            .collect::<Vec<_>>();

        // Parent nodes
        self.json
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.children.as_ref().map(|c| (i, c)))
            .for_each(|(i, children)| {
                let node = &nodes[i];

                children.iter().for_each(|idx| {
                    let child = &nodes[idx.value()];
                    node.add_child(&mut doc.0, child);
                });
            });

        Ok(doc)
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}
