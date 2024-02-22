pub mod accessor;
pub mod buffer;
pub mod document;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;
pub mod texture_info;

pub use accessor::Accessor;
pub use buffer::Buffer;
pub use document::GltfDocument;
pub use image::Image;
pub use material::Material;
pub use mesh::Mesh;
pub use node::Node;
pub use primitive::Primitive;
pub use scene::Scene;
pub use texture_info::TextureInfo;

#[derive(Debug)]
pub enum GltfWeight {
    Accessor(accessor::AccessorWeight),
    Buffer(buffer::BufferWeight),
    Document,
    Image(image::ImageWeight),
    Material(material::MaterialWeight),
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
    TextureInfo(texture_info::TextureInfoWeight),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GltfEdge {
    Accessor(accessor::AccessorEdge),
    Document(document::DocumentEdge),
    Image(image::ImageEdge),
    Material(material::MaterialEdge),
    Mesh(mesh::MeshEdge),
    Node(node::NodeEdge),
    Primitive(primitive::PrimitiveEdge),
    Scene(scene::SceneEdge),
    TextureInfo(texture_info::TextureInfoEdge),
}
