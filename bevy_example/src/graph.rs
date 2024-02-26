use bevy_gltf_kun::import::graph::GltfGraph;
use egui_graphs::Graph;
use gltf_kun::graph::{
    gltf::{
        Accessor, Animation, Buffer, GltfDocument, GltfEdge, GltfWeight, Image, Material, Mesh,
        Node, Scene,
    },
    Edge, Weight,
};
use petgraph::{visit::EdgeRef, Direction};

pub struct GraphSettings {
    pub enable_accessors: bool,
    pub enable_buffers: bool,
    pub enable_document: bool,
    pub enable_images: bool,
    pub enable_materials: bool,
    pub enable_meshes: bool,
    pub enable_nodes: bool,
    pub enable_scenes: bool,
    pub enable_primitives: bool,
    pub enable_texture_infos: bool,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            enable_accessors: false,
            enable_buffers: false,
            enable_document: false,
            enable_images: true,
            enable_materials: true,
            enable_meshes: true,
            enable_nodes: true,
            enable_scenes: true,
            enable_primitives: true,
            enable_texture_infos: true,
        }
    }
}

/// Converts a [GltfGraph] into an [egui_graphs::Graph], adding labels for nicer display.
pub fn create_graph(graph: &GltfGraph, settings: &GraphSettings) -> Graph<Weight, Edge> {
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
                    "Accessor{}",
                    doc.accessor_index(&graph.0, Accessor(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Animation(a)) => a.name.clone().unwrap_or_else(|| {
                format!(
                    "Animation{}",
                    doc.animation_index(&graph.0, Animation(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::AnimationChannel(_)) => "AnimationChannel".to_string(),
            Weight::Gltf(GltfWeight::AnimationSampler(_)) => "AnimationSampler".to_string(),
            Weight::Gltf(GltfWeight::Buffer(b)) => b.name.clone().unwrap_or_else(|| {
                format!(
                    "Buffer{}",
                    doc.buffer_index(&graph.0, Buffer(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Node(n)) => n.name.clone().unwrap_or_else(|| {
                format!("Node{}", doc.node_index(&graph.0, Node(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Scene(s)) => s.name.clone().unwrap_or_else(|| {
                format!("Scene{}", doc.scene_index(&graph.0, Scene(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Image(i)) => i.name.clone().unwrap_or_else(|| {
                format!("Image{}", doc.image_index(&graph.0, Image(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Material(m)) => m.name.clone().unwrap_or_else(|| {
                format!(
                    "Material{}",
                    doc.material_index(&graph.0, Material(*idx)).unwrap()
                )
            }),
            Weight::Gltf(GltfWeight::Mesh(m)) => m.name.clone().unwrap_or_else(|| {
                format!("Mesh{}", doc.mesh_index(&graph.0, Mesh(*idx)).unwrap())
            }),
            Weight::Gltf(GltfWeight::Primitive(_)) => "Primitive".to_string(),
            Weight::Gltf(GltfWeight::TextureInfo(_)) => "TextureInfo".to_string(),

            Weight::Gltf(GltfWeight::Document) => "Document".to_string(),

            Weight::Glxf(_) => "glXF".to_string(),

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
            Edge::Gltf(GltfEdge::Animation(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::AnimationChannel(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::AnimationSampler(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Document(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Image(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Material(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Mesh(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Node(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Primitive(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::Scene(e)) => format!("{:?}", e),
            Edge::Gltf(GltfEdge::TextureInfo(e)) => format!("{:?}", e),

            Edge::Glxf(e) => format!("{:?}", e),

            Edge::Extension(s) => s.to_string(),
            Edge::Other(s) => s.to_string(),
        };

        egui_edge.set_label(label);
    });

    if !settings.enable_document {
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
            g.remove_node(idx);
        }
    }

    if !settings.enable_accessors {
        let accessor_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Accessor(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in accessor_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_buffers {
        let buffer_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Buffer(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in buffer_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_images {
        let image_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Image(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in image_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_materials {
        let material_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Material(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in material_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_meshes {
        let mesh_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Mesh(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in mesh_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_nodes {
        let node_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Node(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in node_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_scenes {
        let scene_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Scene(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in scene_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_primitives {
        let primitive_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::Primitive(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in primitive_indices {
            g.remove_node(idx);
        }
    }

    if !settings.enable_texture_infos {
        let texture_info_indices = g
            .node_indices()
            .filter(|idx| {
                matches!(
                    g.node_weight(*idx).unwrap().payload(),
                    Weight::Gltf(GltfWeight::TextureInfo(_))
                )
            })
            .collect::<Vec<_>>();

        for idx in texture_info_indices {
            g.remove_node(idx);
        }
    }

    egui_graph
}
