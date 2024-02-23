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
}
