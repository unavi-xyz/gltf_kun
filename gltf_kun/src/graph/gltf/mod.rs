pub mod accessor;
pub mod animation;
pub mod buffer;
pub mod document;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod primitive;
pub mod scene;
pub mod skin;
pub mod texture_info;

pub use accessor::Accessor;
pub use animation::Animation;
pub use buffer::Buffer;
pub use document::GltfDocument;
pub use image::Image;
pub use material::Material;
pub use mesh::Mesh;
pub use node::Node;
pub use primitive::Primitive;
pub use scene::Scene;
pub use skin::Skin;
pub use texture_info::TextureInfo;

#[derive(Clone, Debug)]
pub enum GltfWeight {
    Accessor(accessor::AccessorWeight),
    Animation(animation::AnimationWeight),
    AnimationChannel(animation::AnimationChannelWeight),
    AnimationSampler(animation::AnimationSamplerWeight),
    Buffer(buffer::BufferWeight),
    Document,
    Image(image::ImageWeight),
    Material(material::MaterialWeight),
    Mesh(mesh::MeshWeight),
    MorphTarget,
    Node(node::NodeWeight),
    Primitive(primitive::PrimitiveWeight),
    Scene(scene::SceneWeight),
    Skin(skin::SkinWeight),
    TextureInfo(texture_info::TextureInfoWeight),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GltfEdge {
    Accessor(accessor::AccessorEdge),
    Animation(animation::AnimationEdge),
    AnimationChannel(animation::AnimationChannelEdge),
    AnimationSampler(animation::AnimationSamplerEdge),
    Document(document::DocumentEdge),
    Image(image::ImageEdge),
    Material(material::MaterialEdge),
    Mesh(mesh::MeshEdge),
    MorphTarget(primitive::MorphTargetEdge),
    Node(node::NodeEdge),
    Primitive(primitive::PrimitiveEdge),
    Scene(scene::SceneEdge),
    Skin(skin::SkinEdge),
    TextureInfo(texture_info::TextureInfoEdge),
}
