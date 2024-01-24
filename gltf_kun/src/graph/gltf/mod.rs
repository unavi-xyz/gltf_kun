pub mod accessor;
pub mod buffer;
pub mod document;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod sampler;
pub mod scene;
pub mod texture;
pub mod texture_info;

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
    Sampler(sampler::SamplerWeight),
    Scene(scene::SceneWeight),
    Texture(texture::TextureWeight),
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
    Texture(texture::TextureEdge),
    TextureInfo(texture_info::TextureInfoEdge),
}
