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

        self.json.nodes.iter_mut().for_each(|n| {
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
        });

        Ok(doc)
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}

pub struct GlbFormat<'a>(pub gltf::Glb<'a>);

impl<'a> IoFormat for GlbFormat<'a> {
    fn import(mut self) -> Result<Document> {
        let json = serde_json::from_slice(&self.0.json)?;
        let blob = self.0.bin.take().map(|blob| blob.into_owned());

        GltfFormat { json, blob }.import()
    }

    fn export(graph: Document) -> Result<Self> {
        todo!()
    }
}
