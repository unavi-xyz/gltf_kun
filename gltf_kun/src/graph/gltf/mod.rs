pub mod accessor;
pub mod buffer;
pub mod buffer_view;
pub mod document;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;

#[derive(Debug)]
pub enum GltfWeight {
    Accessor(accessor::AccessorWeight),
    Buffer(buffer::BufferWeight),
    BufferView(buffer_view::BufferViewWeight),
    Document,
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Other(Vec<u8>),
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GltfEdge {
    Accessor(accessor::AccessorEdge),
    BufferView(buffer_view::BufferViewEdge),
    Document(document::DocumentEdge),
    Mesh(mesh::MeshEdge),
    Node(node::NodeEdge),
    Primitive(primitive::PrimitiveEdge),
    Scene(scene::SceneEdge),

    Extension(&'static str),
}
