use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction};

use crate::graph::{gltf::GltfEdge, Edge, Graph, Property, Weight};

use super::{
    accessor::Accessor, buffer::Buffer, image::Image, material::Material, mesh::Mesh, node::Node,
    sampler::Sampler, scene::Scene, GltfWeight,
};

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentEdge {
    Accessor,
    Buffer,
    DefaultScene,
    Image,
    Material,
    Mesh,
    Node,
    Sampler,
    Scene,
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

impl Property for GltfDocument {}

impl GltfDocument {
    pub fn new(graph: &mut Graph) -> Self {
        let index = graph.add_node(Weight::Gltf(GltfWeight::Document));
        Self(index)
    }

    fn all_properties<P: Ord + From<NodeIndex>>(
        &self,
        graph: &Graph,
        edge: DocumentEdge,
    ) -> Vec<P> {
        let mut vec = graph
            .edges_directed(self.0, Direction::Outgoing)
            .filter_map(|edge_ref| {
                if let Edge::Gltf(GltfEdge::Document(e)) = edge_ref.weight() {
                    if *e == edge {
                        Some(P::from(edge_ref.target()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        vec.sort();

        vec
    }
    fn add_property(&self, graph: &mut Graph, edge: DocumentEdge, index: NodeIndex) {
        graph.add_edge(self.0, index, Edge::Gltf(GltfEdge::Document(edge)));
    }
    pub fn remove_property(&self, graph: &mut Graph, index: NodeIndex) {
        let edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| edge.target() == index)
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }
    }
    pub fn create_property<P: Copy + Into<NodeIndex> + From<NodeIndex>>(
        &self,
        graph: &mut Graph,
        edge: DocumentEdge,
        weight: GltfWeight,
    ) -> P {
        let property = P::from(graph.add_node(Weight::Gltf(weight)));
        self.add_property(graph, edge, property.into());
        property
    }

    pub fn accessors(&self, graph: &Graph) -> Vec<Accessor> {
        self.all_properties(graph, DocumentEdge::Accessor)
    }
    pub fn add_accessor(&self, graph: &mut Graph, accessor: &Accessor) {
        self.add_property(graph, DocumentEdge::Accessor, accessor.0);
    }
    pub fn remove_accessor(&self, graph: &mut Graph, accessor: &Accessor) {
        self.remove_property(graph, accessor.0);
    }
    pub fn create_accessor(&self, graph: &mut Graph) -> Accessor {
        self.create_property(
            graph,
            DocumentEdge::Accessor,
            GltfWeight::Accessor(Default::default()),
        )
    }

    pub fn buffers(&self, graph: &Graph) -> Vec<Buffer> {
        self.all_properties(graph, DocumentEdge::Buffer)
    }
    pub fn add_buffer(&self, graph: &mut Graph, buffer: &Buffer) {
        self.add_property(graph, DocumentEdge::Buffer, buffer.0);
    }
    pub fn remove_buffer(&self, graph: &mut Graph, buffer: &Buffer) {
        self.remove_property(graph, buffer.0);
    }
    pub fn create_buffer(&self, graph: &mut Graph) -> Buffer {
        self.create_property(
            graph,
            DocumentEdge::Buffer,
            GltfWeight::Buffer(Default::default()),
        )
    }

    pub fn default_scene(&self, graph: &Graph) -> Option<Scene> {
        graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Document(DocumentEdge::DefaultScene))
                )
            })
            .map(|edge| Scene(edge.target()))
    }
    pub fn set_default_scene(&self, graph: &mut Graph, scene: Option<&Scene>) {
        let edge = graph
            .edges_directed(self.0, Direction::Outgoing)
            .find(|edge| {
                matches!(
                    edge.weight(),
                    Edge::Gltf(GltfEdge::Document(DocumentEdge::DefaultScene))
                )
            })
            .map(|edge| edge.id());

        if let Some(edge) = edge {
            graph.remove_edge(edge);
        }

        if let Some(scene) = scene {
            graph.add_edge(
                self.0,
                scene.0,
                Edge::Gltf(GltfEdge::Document(DocumentEdge::DefaultScene)),
            );

            if !self.scenes(graph).contains(scene) {
                self.add_scene(graph, scene);
            }
        }
    }

    pub fn images(&self, graph: &Graph) -> Vec<Image> {
        self.all_properties(graph, DocumentEdge::Image)
    }
    pub fn add_image(&self, graph: &mut Graph, image: &Image) {
        self.add_property(graph, DocumentEdge::Image, image.0);
    }
    pub fn remove_image(&self, graph: &mut Graph, image: &Image) {
        self.remove_property(graph, image.0);
    }
    pub fn create_image(&self, graph: &mut Graph) -> Image {
        self.create_property(
            graph,
            DocumentEdge::Image,
            GltfWeight::Image(Default::default()),
        )
    }

    pub fn materials(&self, graph: &Graph) -> Vec<Material> {
        self.all_properties(graph, DocumentEdge::Material)
    }
    pub fn add_material(&self, graph: &mut Graph, material: &Material) {
        self.add_property(graph, DocumentEdge::Material, material.0);
    }
    pub fn remove_material(&self, graph: &mut Graph, material: &Material) {
        self.remove_property(graph, material.0);
    }
    pub fn create_material(&self, graph: &mut Graph) -> Material {
        self.create_property(
            graph,
            DocumentEdge::Material,
            GltfWeight::Material(Default::default()),
        )
    }

    pub fn meshes(&self, graph: &Graph) -> Vec<Mesh> {
        self.all_properties(graph, DocumentEdge::Mesh)
    }
    pub fn add_mesh(&self, graph: &mut Graph, mesh: &Mesh) {
        self.add_property(graph, DocumentEdge::Mesh, mesh.0);
    }
    pub fn remove_mesh(&self, graph: &mut Graph, mesh: &Mesh) {
        self.remove_property(graph, mesh.0);
    }
    pub fn create_mesh(&self, graph: &mut Graph) -> Mesh {
        self.create_property(
            graph,
            DocumentEdge::Mesh,
            GltfWeight::Mesh(Default::default()),
        )
    }

    pub fn nodes(&self, graph: &Graph) -> Vec<Node> {
        self.all_properties(graph, DocumentEdge::Node)
    }
    pub fn add_node(&self, graph: &mut Graph, node: &Node) {
        self.add_property(graph, DocumentEdge::Node, node.0);
    }
    pub fn remove_node(&self, graph: &mut Graph, node: &Node) {
        self.remove_property(graph, node.0);
    }
    pub fn create_node(&self, graph: &mut Graph) -> Node {
        self.create_property(
            graph,
            DocumentEdge::Node,
            GltfWeight::Node(Default::default()),
        )
    }

    pub fn samplers(&self, graph: &Graph) -> Vec<Sampler> {
        self.all_properties(graph, DocumentEdge::Sampler)
    }
    pub fn add_sampler(&self, graph: &mut Graph, sampler: &Accessor) {
        self.add_property(graph, DocumentEdge::Sampler, sampler.0);
    }
    pub fn remove_sampler(&self, graph: &mut Graph, sampler: &Accessor) {
        self.remove_property(graph, sampler.0);
    }
    pub fn create_sampler(&self, graph: &mut Graph) -> Accessor {
        self.create_property(
            graph,
            DocumentEdge::Sampler,
            GltfWeight::Accessor(Default::default()),
        )
    }

    pub fn scenes(&self, graph: &Graph) -> Vec<Scene> {
        self.all_properties(graph, DocumentEdge::Scene)
    }
    pub fn add_scene(&self, graph: &mut Graph, scene: &Scene) {
        self.add_property(graph, DocumentEdge::Scene, scene.0);
    }
    pub fn remove_scene(&self, graph: &mut Graph, scene: &Scene) {
        self.remove_property(graph, scene.0);
    }
    pub fn create_scene(&self, graph: &mut Graph) -> Scene {
        self.create_property(
            graph,
            DocumentEdge::Scene,
            GltfWeight::Scene(Default::default()),
        )
    }
}
