use std::collections::HashMap;
use std::sync::Arc;

mod graph;

use graph::*;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

pub struct Node {
    graph: Arc<GltfGraph>,
    index: NodeIndex,
}

impl Node {
    pub fn data(&self) -> &NodeData {
        match &self.graph[self.index] {
            GraphNode::Node(node) => node,
            _ => panic!("Node is not a NodeData"),
        }
    }

    pub fn children(&self) -> Vec<Node> {
        self.graph
            .edges(self.index)
            .filter_map(|edge| {
                let index = match edge.weight() {
                    GraphEdge::Child => edge.target(),
                    _ => return None,
                };

                Some(Node {
                    graph: self.graph.clone(),
                    index,
                })
            })
            .collect()
    }
}

pub struct Gltf {
    graph: Arc<GltfGraph>,
}

impl Gltf {
    /// Create a new Gltf from json
    pub fn from_json(json: &gltf::json::Root) -> Self {
        let mut graph = GltfGraph::new();
        let mut nodes = HashMap::new();

        json.nodes.iter().enumerate().for_each(|(i, node)| {
            let graph_node = graph.add_node(GraphNode::Node(NodeData {
                name: node.name.clone(),
                translation: node.translation.unwrap_or([0.0, 0.0, 0.0]),
                rotation: node.rotation.unwrap_or_default().0,
                scale: node.scale.unwrap_or([1.0, 1.0, 1.0]),
            }));

            nodes.insert(i, graph_node);
        });

        json.nodes.iter().enumerate().for_each(|(i, node)| {
            let graph_node = nodes.get(&i).unwrap();

            if let Some(children) = &node.children {
                children.iter().for_each(|child| {
                    let child_graph_node = nodes.get(&child.value()).unwrap();

                    graph.add_edge(*graph_node, *child_graph_node, GraphEdge::Child);
                    graph.add_edge(*child_graph_node, *graph_node, GraphEdge::Parent);
                });
            }
        });

        Gltf {
            graph: Arc::new(graph),
        }
    }

    /// Get all glTF nodes
    pub fn nodes(&self) -> Vec<Node> {
        self.graph
            .node_indices()
            .filter_map(|index| match self.graph[index] {
                GraphNode::Node(_) => Some(Node {
                    graph: self.graph.clone(),
                    index,
                }),
                _ => None,
            })
            .collect()
    }
}

/// Import a glTF from the file system
pub fn import(path: &str) -> Result<Gltf, gltf::Error> {
    let (doc, _, _) = gltf::import(path)?;
    Ok(Gltf::from_json(&doc.into_json()))
}
