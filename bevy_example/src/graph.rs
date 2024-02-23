use bevy_gltf_kun::import::graph::GltfGraph;
use egui_graphs::Graph;
use gltf_kun::graph::{
    gltf::{
        Accessor, Buffer, GltfDocument, GltfEdge, GltfWeight, Image, Material, Mesh, Node, Scene,
    },
    Edge, Weight,
};
use petgraph::{visit::EdgeRef, Direction};

pub struct GraphSettings {
    pub hide_document: bool,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            hide_document: true,
        }
    }
}

pub fn create_graph(graph: &GltfGraph, settings: GraphSettings) -> Graph<Weight, Edge> {
    let mut egui_graph = Graph::from(&graph.0);

    let g = egui_graph.g();

    let doc = g
        .node_indices()
        .find(|idx| {
            let weight = match g.node_weight(*idx) {
                Some(weight) => weight,
                None => return false,
            };

            matches!(weight.payload(), Weight::Gltf(GltfWeight::Document))
        })
        .map(GltfDocument);

    let doc = match doc {
        Some(doc) => doc,
        None => return egui_graph,
    };

    let node_indices = g.node_indices().collect::<Vec<_>>();
    let edge_indices = g.edge_indices().collect::<Vec<_>>();

    node_indices.iter().for_each(|idx| {
        let egui_node = g.node_weight_mut(*idx).unwrap();

        let label = match egui_node.payload() {
            Weight::Gltf(GltfWeight::Accessor(a)) => a.name.clone().unwrap_or_else(|| {
                format!(
                    "Accessor {}",
                    doc.accessor_index(&graph.0, Accessor(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Buffer(b)) => b.name.clone().unwrap_or_else(|| {
                format!(
                    "Buffer {}",
                    doc.buffer_index(&graph.0, Buffer(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Node(n)) => n.name.clone().unwrap_or_else(|| {
                format!("Node {}", doc.node_index(&graph.0, Node(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Scene(s)) => s.name.clone().unwrap_or_else(|| {
                format!("Scene {}", doc.scene_index(&graph.0, Scene(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Image(i)) => i.name.clone().unwrap_or_else(|| {
                format!("Image {}", doc.image_index(&graph.0, Image(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Material(m)) => m.name.clone().unwrap_or_else(|| {
                format!(
                    "Material {}",
                    doc.material_index(&graph.0, Material(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Mesh(m)) => m.name.clone().unwrap_or_else(|| {
                format!("Mesh {}", doc.mesh_index(&graph.0, Mesh(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Primitive(_)) => "Primitive".to_string(),
            Weight::Gltf(GltfWeight::TextureInfo(_)) => "TextureInfo".to_string(),

            Weight::Gltf(GltfWeight::Document) => "Document".to_string(),

            Weight::Glxf(_) => "Glxf".to_string(),
            Weight::Bytes(_) => {
                let extension = g
                    .edges_directed(*idx, Direction::Incoming)
                    .find_map(|edge_idx| {
                        let edge = g.edge_weight(edge_idx.id()).unwrap();
                        match edge.payload() {
                            Edge::Extension(s) => Some(s),
                            _ => None,
                        }
                    })
                    .map(|s| s.to_string());

                extension.unwrap_or("Unknown Bytes".to_string())
            }
        };

        let egui_node = g.node_weight_mut(*idx).unwrap();
        egui_node.set_label(label);
    });

    edge_indices.iter().for_each(|idx| {
        let egui_edge = g.edge_weight_mut(*idx).unwrap();

        let label = match egui_edge.payload() {
            Edge::Gltf(GltfEdge::Accessor(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Image(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Mesh(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Node(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Scene(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Document(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Material(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Primitive(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::TextureInfo(e)) => format!("{:?}", e),

            Edge::Extension(s) => s.to_string(),
            Edge::Other(s) => s.to_string(),
            Edge::Glxf(_) => "Glxf".to_string(),
        };

        egui_edge.set_label(label);
    });

    if settings.hide_document {
        let document_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Document)
                )
            })
            .collect::<Vec<_>>();

        for idx in document_indices {
            egui_graph.g().remove_node(idx);
        }
    }

    egui_graph
}
