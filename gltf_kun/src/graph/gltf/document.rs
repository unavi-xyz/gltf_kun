use petgraph::graph::NodeIndex;

use crate::graph::{gltf::GltfEdge, Edge, Graph, GraphNodeEdges, Property, Weight};

use super::{
    accessor::Accessor, buffer::Buffer, image::Image, material::Material, mesh::Mesh, node::Node,
    scene::Scene, GltfWeight,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentEdge {
    Accessor,
    Buffer,
    DefaultScene,
    Image,
    Material,
    Mesh,
    Node,
    Scene,
}

impl<'a> TryFrom<&'a Edge> for &'a DocumentEdge {
    type Error = ();
    fn try_from(value: &'a Edge) -> Result<Self, Self::Error> {
        match value {
            Edge::Gltf(GltfEdge::Document(edge)) => Ok(edge),
            _ => Err(()),
        }
    }
}

impl From<DocumentEdge> for Edge {
    fn from(edge: DocumentEdge) -> Self {
        Self::Gltf(GltfEdge::Document(edge))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GltfDocument(pub NodeIndex);

impl From<NodeIndex> for GltfDocument {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<GltfDocument> for NodeIndex {
    fn from(document: GltfDocument) -> Self {
        document.0
    }
}

impl GraphNodeEdges<DocumentEdge> for GltfDocument {}
impl Property for GltfDocument {}

impl GltfDocument {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Document));
        Self(index)
    }

    pub fn accessors(&self, graph: &Graph) -> Vec<Accessor> {
        self.edge_targets(graph, &DocumentEdge::Accessor)
    }
    pub fn add_accessor(&self, graph: &mut Graph, accessor: Accessor) {
        self.add_edge_target(graph, DocumentEdge::Accessor, accessor);
    }
    pub fn remove_accessor(&self, graph: &mut Graph, accessor: Accessor) {
        self.remove_edge_target(graph, DocumentEdge::Accessor, accessor);
    }
    pub fn create_accessor(&self, graph: &mut Graph) -> Accessor {
        self.create_edge_target(graph, DocumentEdge::Accessor)
    }
    pub fn accessor_index(&self, graph: &Graph, accessor: Accessor) -> Option<usize> {
        self.accessors(graph).iter().position(|a| *a == accessor)
    }

    pub fn buffers(&self, graph: &Graph) -> Vec<Buffer> {
        self.edge_targets(graph, &DocumentEdge::Buffer)
    }
    pub fn add_buffer(&self, graph: &mut Graph, buffer: Buffer) {
        self.add_edge_target(graph, DocumentEdge::Buffer, buffer);
    }
    pub fn remove_buffer(&self, graph: &mut Graph, buffer: Buffer) {
        self.remove_edge_target(graph, DocumentEdge::Buffer, buffer);
    }
    pub fn create_buffer(&self, graph: &mut Graph) -> Buffer {
        self.create_edge_target(graph, DocumentEdge::Buffer)
    }
    pub fn buffer_index(&self, graph: &Graph, buffer: Buffer) -> Option<usize> {
        self.buffers(graph).iter().position(|b| *b == buffer)
    }

    pub fn default_scene(&self, graph: &Graph) -> Option<Scene> {
        self.find_edge_target(graph, &DocumentEdge::DefaultScene)
    }
    pub fn set_default_scene(&self, graph: &mut Graph, scene: Option<Scene>) {
        self.set_edge_target(graph, DocumentEdge::DefaultScene, scene);
    }

    pub fn images(&self, graph: &Graph) -> Vec<Image> {
        self.edge_targets(graph, &DocumentEdge::Image)
    }
    pub fn add_image(&self, graph: &mut Graph, image: Image) {
        self.add_edge_target(graph, DocumentEdge::Image, image);
    }
    pub fn remove_image(&self, graph: &mut Graph, image: Image) {
        self.remove_edge_target(graph, DocumentEdge::Image, image);
    }
    pub fn create_image(&self, graph: &mut Graph) -> Image {
        self.create_edge_target(graph, DocumentEdge::Image)
    }
    pub fn image_index(&self, graph: &Graph, image: Image) -> Option<usize> {
        self.images(graph).iter().position(|i| *i == image)
    }

    pub fn materials(&self, graph: &Graph) -> Vec<Material> {
        self.edge_targets(graph, &DocumentEdge::Material)
    }
    pub fn add_material(&self, graph: &mut Graph, material: Material) {
        self.add_edge_target(graph, DocumentEdge::Material, material);
    }
    pub fn remove_material(&self, graph: &mut Graph, material: Material) {
        self.remove_edge_target(graph, DocumentEdge::Material, material);
    }
    pub fn create_material(&self, graph: &mut Graph) -> Material {
        self.create_edge_target(graph, DocumentEdge::Material)
    }
    pub fn material_index(&self, graph: &Graph, material: Material) -> Option<usize> {
        self.materials(graph).iter().position(|m| *m == material)
    }

    pub fn meshes(&self, graph: &Graph) -> Vec<Mesh> {
        self.edge_targets(graph, &DocumentEdge::Mesh)
    }
    pub fn add_mesh(&self, graph: &mut Graph, mesh: Mesh) {
        self.add_edge_target(graph, DocumentEdge::Mesh, mesh);
    }
    pub fn remove_mesh(&self, graph: &mut Graph, mesh: Mesh) {
        self.remove_edge_target(graph, DocumentEdge::Mesh, mesh);
    }
    pub fn create_mesh(&self, graph: &mut Graph) -> Mesh {
        self.create_edge_target(graph, DocumentEdge::Mesh)
    }
    pub fn mesh_index(&self, graph: &Graph, mesh: Mesh) -> Option<usize> {
        self.meshes(graph).iter().position(|m| *m == mesh)
    }

    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        self.edge_targets(graph, &DocumentEdge::Node)
    }
    pub fn add_node(&self, graph: &mut Graph, node: Node) {
        self.add_edge_target(graph, DocumentEdge::Node, node);
    }
    pub fn remove_node(&self, graph: &mut Graph, node: Node) {
        self.remove_edge_target(graph, DocumentEdge::Node, node);
    }
    pub fn create_node(&self, graph: &mut Graph) -> Node {
        self.create_edge_target(graph, DocumentEdge::Node)
    }
    pub fn node_index(&self, graph: &Graph, node: Node) -> Option<usize> {
        self.nodes(graph).iter().position(|n| *n == node)
    }

    pub fn scenes(&self, graph: &Graph) -> Vec<Scene> {
        self.edge_targets(graph, &DocumentEdge::Scene)
    }
    pub fn add_scene(&self, graph: &mut Graph, scene: Scene) {
        self.add_edge_target(graph, DocumentEdge::Scene, scene);
    }
    pub fn remove_scene(&self, graph: &mut Graph, scene: Scene) {
        self.remove_edge_target(graph, DocumentEdge::Scene, scene);
    }
    pub fn create_scene(&self, graph: &mut Graph) -> Scene {
        self.create_edge_target(graph, DocumentEdge::Scene)
    }
    pub fn scene_index(&self, graph: &Graph, scene: Scene) -> Option<usize> {
        self.scenes(graph).iter().position(|s| *s == scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_methods() {
        let graph = &mut Graph::new();
        let doc = GltfDocument::new(graph);

        let a = doc.create_accessor(graph);
        let a_2 = doc.create_accessor(graph);
        assert_eq!(doc.accessor_index(graph, a), Some(0));
        assert_eq!(doc.accessor_index(graph, a_2), Some(1));
        doc.remove_accessor(graph, a);
        assert_eq!(doc.accessor_index(graph, a), None);
        assert_eq!(doc.accessor_index(graph, a_2), Some(0));

        let b = doc.create_buffer(graph);
        let b_2 = doc.create_buffer(graph);
        assert_eq!(doc.buffer_index(graph, b), Some(0));
        assert_eq!(doc.buffer_index(graph, b_2), Some(1));
        doc.remove_buffer(graph, b);
        assert_eq!(doc.buffer_index(graph, b), None);
        assert_eq!(doc.buffer_index(graph, b_2), Some(0));

        let i = doc.create_image(graph);
        let i_2 = doc.create_image(graph);
        assert_eq!(doc.image_index(graph, i), Some(0));
        assert_eq!(doc.image_index(graph, i_2), Some(1));
        doc.remove_image(graph, i);
        assert_eq!(doc.image_index(graph, i), None);
        assert_eq!(doc.image_index(graph, i_2), Some(0));

        let m = doc.create_material(graph);
        let m_2 = doc.create_material(graph);
        assert_eq!(doc.material_index(graph, m), Some(0));
        assert_eq!(doc.material_index(graph, m_2), Some(1));
        doc.remove_material(graph, m);
        assert_eq!(doc.material_index(graph, m), None);
        assert_eq!(doc.material_index(graph, m_2), Some(0));

        let me = doc.create_mesh(graph);
        let me_2 = doc.create_mesh(graph);
        assert_eq!(doc.mesh_index(graph, me), Some(0));
        assert_eq!(doc.mesh_index(graph, me_2), Some(1));
        doc.remove_mesh(graph, me);
        assert_eq!(doc.mesh_index(graph, me), None);
        assert_eq!(doc.mesh_index(graph, me_2), Some(0));

        let n = doc.create_node(graph);
        let n_2 = doc.create_node(graph);
        assert_eq!(doc.node_index(graph, n), Some(0));
        assert_eq!(doc.node_index(graph, n_2), Some(1));
        doc.remove_node(graph, n);
        assert_eq!(doc.node_index(graph, n), None);
        assert_eq!(doc.node_index(graph, n_2), Some(0));

        let s = doc.create_scene(graph);
        let s_2 = doc.create_scene(graph);
        assert_eq!(doc.scene_index(graph, s), Some(0));
        assert_eq!(doc.scene_index(graph, s_2), Some(1));
        doc.remove_scene(graph, s);
        assert_eq!(doc.scene_index(graph, s), None);
        assert_eq!(doc.scene_index(graph, s_2), Some(0));
    }
}
