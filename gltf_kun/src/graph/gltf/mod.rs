pub mod accessor;
pub mod buffer;
pub mod buffer_view;
pub mod document;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;
pub mod texture;
pub mod texture_info;

#[derive(Debug)]
pub enum GltfWeight {
    Accessor(accessor::AccessorWeight),
    Buffer(buffer::BufferWeight),
    BufferView(buffer_view::BufferViewWeight),
    Document,
    Image(image::ImageWeight),
    Material(material::MaterialWeight),
    Mesh(mesh::MeshWeight),
    Node(node::NodeWeight),
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
    Texture(texture::TextureWeight),
    TextureInfo(texture_info::TextureInfoWeight),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GltfEdge {
    Accessor(accessor::AccessorEdge),
    BufferView(buffer_view::BufferViewEdge),
    Document(document::DocumentEdge),
    Image(image::ImageEdge),
    Material(material::MaterialEdge),
    Mesh(mesh::MeshEdge),
    Node(node::NodeEdge),
    Primitive(primitive::PrimitiveEdge),
    Scene(scene::SceneEdge),
    Texture(texture::TextureEdge),
    TextureInfo(texture_info::TextureInfoEdge),
}
