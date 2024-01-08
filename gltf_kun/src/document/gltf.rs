use crate::graph::gltf::{
    accessor::Accessor, buffer::Buffer, buffer_view::BufferView, mesh::Mesh, node::Node,
    primitive::Primitive, scene::Scene, Edge, GltfGraph, Weight,
};

#[derive(Default)]
pub struct GltfDocument(pub GltfGraph);

impl GltfDocument {
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

    pub fn create_accessor(&mut self) -> Accessor {
        Accessor(self.0.add_node(Weight::Accessor(Default::default())))
    }
    pub fn create_buffer(&mut self) -> Buffer {
        Buffer(self.0.add_node(Weight::Buffer(Default::default())))
    }
    pub fn create_buffer_view(&mut self) -> BufferView {
        BufferView(self.0.add_node(Weight::BufferView(Default::default())))
    }
    pub fn create_mesh(&mut self) -> Mesh {
        Mesh(self.0.add_node(Weight::Mesh(Default::default())))
    }
    pub fn create_node(&mut self) -> Node {
        Node(self.0.add_node(Weight::Node(Default::default())))
    }
    pub fn create_primitive(&mut self) -> Primitive {
        Primitive(self.0.add_node(Weight::Primitive(Default::default())))
    }
    pub fn create_scene(&mut self) -> Scene {
        Scene(self.0.add_node(Weight::Scene(Default::default())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let graph = GltfGraph::default();
        let mut doc = GltfDocument(graph);

        let accessor = doc.create_accessor();
        assert_eq!(doc.accessors().len(), 1);
        assert_eq!(doc.accessors()[0], accessor);

        let buffer = doc.create_buffer();
        assert_eq!(doc.buffers().len(), 1);
        assert_eq!(doc.buffers()[0], buffer);

        let buffer_view = doc.create_buffer_view();
        assert_eq!(doc.buffer_views().len(), 1);
        assert_eq!(doc.buffer_views()[0], buffer_view);

        let mesh = doc.create_mesh();
        assert_eq!(doc.meshes().len(), 1);
        assert_eq!(doc.meshes()[0], mesh);

        let node = doc.create_node();
        assert_eq!(doc.nodes().len(), 1);
        assert_eq!(doc.nodes()[0], node);

        let primitive = doc.create_primitive();
        assert_eq!(doc.primitives().len(), 1);
        assert_eq!(doc.primitives()[0], primitive);

        let scene = doc.create_scene();
        assert_eq!(doc.scenes().len(), 1);
        assert_eq!(doc.scenes()[0], scene);
    }

    #[test]
    fn test_default_scene() {
        let graph = GltfGraph::default();
        let mut doc = GltfDocument(graph);

        let scene = doc.create_scene();
        doc.set_default_scene(Some(&scene));
        assert_eq!(doc.default_scene(), Some(scene));

        doc.set_default_scene(None);
        assert_eq!(doc.default_scene(), None);
    }
}
