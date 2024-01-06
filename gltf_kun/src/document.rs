use crate::graph::{
    accessor::Accessor, buffer::Buffer, buffer_view::BufferView, mesh::Mesh, node::Node,
    primitive::Primitive, scene::Scene, Edge, GltfGraph, Weight,
};

#[derive(Default)]
pub struct Document(pub GltfGraph);

impl Document {
    pub fn default_scene(&self) -> Option<Scene> {
        self.0
            .node_indices()
            .find(|weight| matches!(self.0[*weight], Weight::DefaultScene))
            .and_then(|index| {
                self.0
                    .neighbors_directed(index, petgraph::Direction::Outgoing)
                    .find_map(|index| match self.0[index] {
                        Weight::Scene(_) => Some(Scene(index)),
                        _ => None,
                    })
            })
    }
    pub fn set_default_scene(&mut self, scene: Option<&Scene>) {
        let index = self
            .0
            .node_indices()
            .find(|weight| matches!(self.0[*weight], Weight::DefaultScene));

        if let Some(idx) = index {
            self.0.remove_node(idx);
        }

        if let Some(scene) = scene {
            let default_scene = self.0.add_node(Weight::DefaultScene);
            self.0.add_edge(default_scene, scene.0, Edge::Scene);
        }
    }

    pub fn accessors(&self) -> Vec<Accessor> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Accessor(_)))
            .map(Accessor)
            .collect()
    }

    pub fn buffers(&self) -> Vec<Buffer> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Buffer(_)))
            .map(Buffer)
            .collect()
    }

    pub fn buffer_views(&self) -> Vec<BufferView> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::BufferView(_)))
            .map(BufferView)
            .collect()
    }

    pub fn meshes(&self) -> Vec<Mesh> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Mesh(_)))
            .map(Mesh)
            .collect()
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Node(_)))
            .map(Node)
            .collect()
    }

    pub fn primitives(&self) -> Vec<Primitive> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Primitive(_)))
            .map(Primitive)
            .collect()
    }

    pub fn scenes(&self) -> Vec<Scene> {
        self.0
            .node_indices()
            .filter(|weight| matches!(self.0[*weight], Weight::Scene(_)))
            .map(Scene)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::Node;

    #[test]
    fn test_node() {
        let graph = GltfGraph::default();
        let mut doc = Document(graph);

        let scene = Scene::new(&mut doc.0);

        let scenes = doc.scenes();
        assert_eq!(scenes.len(), 1);
        assert_eq!(scenes[0], scene);

        doc.set_default_scene(Some(&scene));
        assert_eq!(doc.default_scene(), Some(scene));

        let node = Node::new(&mut doc.0);
        scene.add_node(&mut doc.0, &node);

        let nodes = doc.nodes();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0], node);
    }
}
