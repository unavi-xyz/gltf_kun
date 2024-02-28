use std::{borrow::Cow, collections::HashMap, path::Path};

use petgraph::{visit::EdgeRef, Direction};
use thiserror::Error;

use crate::{
    extensions::ExtensionsIO,
    graph::{gltf::document::GltfDocument, Graph},
    io::resolver::DataUriResolver,
};

use super::gltf::{export::GltfExportError, import::GltfImportError, GltfFormat, GltfIO};

#[derive(Default)]
pub struct GlbFormat(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("Failed to import gltf: {0}")]
    Import(#[from] GlbImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

pub struct GlbIO<E: ExtensionsIO<GltfDocument, GltfFormat>> {
    pub _marker: std::marker::PhantomData<E>,
}

#[derive(Debug, Error)]
pub enum GlbExportError {
    #[error("Failed to export gltf: {0}")]
    Export(#[from] GltfExportError),
    #[error("Failed to export glb: {0}")]
    Gltf(#[from] gltf::Error),
    #[error("Glb only supports one buffer")]
    MultipleBuffers,
    #[error("Failed to serialize json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum GlbImportError {
    #[error("Failed to parse glb: {0}")]
    Gltf(#[from] gltf::Error),
    #[error("Failed to import gltf: {0}")]
    Import(#[from] GltfImportError),
    #[error("Failed to parse json: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl<E: ExtensionsIO<GltfDocument, GltfFormat>> GlbIO<E> {
    pub fn export(graph: &mut Graph, doc: &GltfDocument) -> Result<GlbFormat, GlbExportError> {
        let buffers = doc.buffers(graph);

        if buffers.len() > 1 {
            let buffer = buffers.first().unwrap();

            buffers.iter().skip(1).for_each(|b| {
                // Remove buffer from the document.
                doc.remove_buffer(graph, *b);

                // Direct all edges to the first buffer.
                let edges = graph
                    .edges_directed(b.0, Direction::Incoming)
                    .map(|edge| (edge.weight().clone(), edge.source()))
                    .collect::<Vec<_>>();

                for (weight, source) in edges {
                    graph.add_edge(source, buffer.0, weight);
                }

                // Remove buffer from the graph.
                graph.remove_node(b.0);
            });
        }

        // Set the buffer for all images to the first buffer.
        for image in doc.images(graph) {
            let buffer = buffers.first().unwrap();
            image.set_buffer(graph, Some(*buffer));
        }

        let gltf = GltfIO::<E>::export(graph, doc)?;

        let json_bin = gltf.json.to_vec()?;
        let bin = gltf.resources.values().next();

        let length = json_bin.len() + bin.map(|b| b.len()).unwrap_or(0);

        let glb = gltf::Glb {
            header: gltf::binary::Header {
                magic: *b"glTF",
                version: 2,
                length: length as u32,
            },
            json: Cow::Owned(json_bin),
            bin: bin.map(|b| b.into()),
        };

        let bytes = glb.to_vec()?;

        Ok(GlbFormat(bytes))
    }

    pub async fn import_slice(
        graph: &mut Graph,
        bytes: &[u8],
    ) -> Result<GltfDocument, GlbImportError> {
        let format = GlbFormat(bytes.to_vec());
        Self::import(graph, format).await
    }

    pub async fn import_file(
        graph: &mut Graph,
        path: &Path,
    ) -> Result<GltfDocument, ImportFileError> {
        let bytes = std::fs::read(path)?;
        let doc = Self::import_slice(graph, &bytes).await?;
        Ok(doc)
    }

    pub async fn import(
        graph: &mut Graph,
        format: GlbFormat,
    ) -> Result<GltfDocument, GlbImportError> {
        let mut glb = gltf::Glb::from_slice(&format.0)?;

        let json = serde_json::from_slice(&glb.json)?;
        let bin = glb.bin.take().map(|bin| bin.into_owned());

        let mut resources = HashMap::new();

        if let Some(bin) = bin {
            resources.insert("bin".to_string(), bin);
        }

        let format = GltfFormat { json, resources };
        let doc = GltfIO::<E>::import(graph, format, None::<DataUriResolver>).await?;

        Ok(doc)
    }
}
