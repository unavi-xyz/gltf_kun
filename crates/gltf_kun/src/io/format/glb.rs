use std::{borrow::Cow, collections::HashMap, path::Path};

use petgraph::{Direction, visit::EdgeRef};
use thiserror::Error;

use crate::{
    extensions::{ExtensionExport, ExtensionImport},
    graph::{Graph, gltf::document::GltfDocument},
    io::resolver::DataUriResolver,
};

use super::gltf::{
    GltfExport, GltfFormat, GltfImport, export::GltfExportError, import::GltfImportError,
};

#[derive(Default)]
pub struct GlbFormat(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum ImportFileError {
    #[error("Failed to import gltf: {0}")]
    Import(#[from] GlbImportError),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
}

pub struct GlbExport<E>
where
    E: ExtensionExport<GltfDocument, GltfFormat>,
{
    pub _marker: std::marker::PhantomData<E>,
}

pub struct GlbImport<E>
where
    E: ExtensionImport<GltfDocument, GltfFormat>,
{
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

impl<E> GlbExport<E>
where
    E: ExtensionExport<GltfDocument, GltfFormat>,
{
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
            let buffer = if buffers.is_empty() {
                doc.create_buffer(graph)
            } else {
                *buffers.first().unwrap()
            };

            image.set_buffer(graph, Some(buffer));
        }

        let mut gltf = GltfExport::<E>::export(graph, doc)?;

        // Remove the buffer URI.
        for buf in gltf.json.buffers.iter_mut() {
            buf.uri = None;
        }

        let json_bin = gltf.json.to_vec()?;
        let bin = gltf.resources.values().next();

        let glb = gltf::Glb {
            header: gltf::binary::Header {
                magic: *b"glTF",
                version: 2,
                // Set automatically when serialized.
                length: 0,
            },
            json: Cow::Owned(json_bin),
            bin: bin.map(|b| b.into()),
        };

        let bytes = glb.to_vec()?;

        Ok(GlbFormat(bytes))
    }
}

impl<E> GlbImport<E>
where
    E: ExtensionImport<GltfDocument, GltfFormat>,
{
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
        let doc = GltfImport::<E>::import(graph, format, None::<DataUriResolver>).await?;

        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::DefaultExtensions, graph::GraphNodeWeight};

    use super::*;

    #[tokio::test]
    async fn test_multiple_buffers() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        doc.create_buffer(&mut graph);
        doc.create_buffer(&mut graph);
        doc.create_buffer(&mut graph);

        let bytes = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).unwrap();
        let gltf = GlbImport::<DefaultExtensions>::import_slice(&mut graph, &bytes.0)
            .await
            .unwrap();

        assert_eq!(gltf.buffers(&graph).len(), 1);
    }

    #[tokio::test]
    async fn test_set_image_buffer() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        {
            doc.create_buffer(&mut graph);

            let mut image = doc.create_image(&mut graph);
            let image_weight = image.get_mut(&mut graph);
            image_weight.data = vec![0, 1, 2, 3];
        }

        let bytes = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).unwrap();
        let gltf = GlbImport::<DefaultExtensions>::import_slice(&mut graph, &bytes.0)
            .await
            .unwrap();

        assert_eq!(gltf.buffers(&graph).len(), 1);

        let buffer = *gltf.buffers(&graph).first().unwrap();
        let image = *gltf.images(&graph).first().unwrap();
        assert_eq!(image.buffer(&graph), Some(buffer));

        let image_weight = image.get(&graph);
        assert_eq!(image_weight.data, vec![0, 1, 2, 3]);
    }

    #[tokio::test]
    async fn test_no_buffer() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        {
            let mut image = doc.create_image(&mut graph);
            let image_weight = image.get_mut(&mut graph);
            image_weight.data = vec![0, 1, 2, 3];
        }

        let bytes = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).unwrap();
        let gltf = GlbImport::<DefaultExtensions>::import_slice(&mut graph, &bytes.0)
            .await
            .unwrap();

        assert_eq!(gltf.buffers(&graph).len(), 1);

        let buffer = *gltf.buffers(&graph).first().unwrap();
        let image = *gltf.images(&graph).first().unwrap();
        assert_eq!(image.buffer(&graph), Some(buffer));

        let image_weight = image.get(&graph);
        assert_eq!(image_weight.data, vec![0, 1, 2, 3]);
    }

    #[tokio::test]
    async fn test_header_length() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let mut image = doc.create_image(&mut graph);
        let image_weight = image.get_mut(&mut graph);
        image_weight.data = vec![0, 1, 2, 3];

        let mut accessor = doc.create_accessor(&mut graph);
        let accessor_weight = accessor.get_mut(&mut graph);
        accessor_weight.data = vec![7; 256];

        let bytes = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).unwrap();
        let glb = gltf::Glb::from_slice(&bytes.0).unwrap();
        assert_eq!(glb.header.length, glb.to_vec().unwrap().len() as u32);
    }

    #[tokio::test]
    async fn test_no_uri() {
        let mut graph = Graph::default();
        let doc = GltfDocument::new(&mut graph);

        let mut image = doc.create_image(&mut graph);
        let image_weight = image.get_mut(&mut graph);
        image_weight.data = vec![0, 1, 2, 3];

        let mut accessor = doc.create_accessor(&mut graph);
        let accessor_weight = accessor.get_mut(&mut graph);
        accessor_weight.data = vec![7; 256];

        let bytes = GlbExport::<DefaultExtensions>::export(&mut graph, &doc).unwrap();
        let out = gltf::Gltf::from_slice(&bytes.0).unwrap();

        let buffers = out.buffers();
        assert_eq!(buffers.len(), 1);

        for b in buffers {
            let source = b.source();
            println!("source={source:?}");
            assert!(matches!(source, gltf::buffer::Source::Bin));
        }
    }
}
