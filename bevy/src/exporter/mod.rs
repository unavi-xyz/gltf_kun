use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use self::utils::name_to_string;
use gltf_kun::{
    accessor::Accessor,
    graph::{AttributeSemantic, ElementType, PrimitiveMode},
    node::Node,
    Gltf,
};

mod utils;

pub fn export_gltf(
    mut events: EventReader<super::ExportScene>,
    meshes: Res<Assets<Mesh>>,
    scenes: Query<(Option<&Name>, Option<&Children>), With<Handle<Scene>>>,
    nodes_query: Query<(&Transform, Option<&Name>, Option<&Children>)>,
    meshes_query: Query<(&Handle<Mesh>, Option<&Name>)>,
) {
    for event in events.read() {
        let mut gltf = gltf_kun::Gltf::default();

        for scene in &event.scenes {
            let (name, children) = match scenes.get(*scene) {
                Ok(scene) => scene,
                Err(_) => {
                    error!("Scene not found");
                    continue;
                }
            };

            let mut scene = gltf.create_scene();
            scene.set_name(name_to_string(name));

            let children = match children {
                Some(children) => children.to_vec(),
                None => Vec::new(),
            };

            children.iter().for_each(|entity| {
                let mut node = export_node(entity, &mut gltf, &nodes_query, &meshes_query, &meshes);
                scene.add_node(&mut node);
            })
        }

        let (json, _) = gltf.to_json();
        println!("EXPORTED: {:#?}", json);
    }
}

fn export_node(
    entity: &Entity,
    gltf: &mut gltf_kun::Gltf,
    nodes_query: &Query<(&Transform, Option<&Name>, Option<&Children>)>,
    meshes_query: &Query<(&Handle<Mesh>, Option<&Name>)>,
    meshes: &Assets<Mesh>,
) -> Node {
    let (transform, name, children) = match nodes_query.get(*entity) {
        Ok(node) => node,
        Err(_) => panic!("Node not found"),
    };

    let mut node = gltf.create_node();

    let mesh = match meshes_query.get(*entity) {
        Ok((handle, mesh_name)) => {
            let mut mesh = gltf.create_mesh();
            mesh.set_name(name_to_string(mesh_name));

            let mut primitive = mesh.create_primitive();
            primitive.set_mode(PrimitiveMode::Triangles);

            let asset = meshes.get(handle).unwrap();

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_POSITION) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Position);
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Normal);
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_TANGENT) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Tangent);
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_UV_0) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::TexCoord(0));
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_UV_1) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::TexCoord(1));
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_COLOR) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Color(0));
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_JOINT_INDEX) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Joints(0));
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(attr) = asset.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_JOINT_WEIGHT) {
                let accessor = attribute_to_accessor(attr, gltf);

                if let Ok(accessor) = accessor {
                    let mut attribute = primitive.create_attribute(AttributeSemantic::Weights(0));
                    attribute.set_accessor(Some(accessor));
                }
            }

            if let Some(indices) = asset.indices() {
                let mut accessor = gltf.create_accessor();
                accessor.set_array(indices.iter().collect::<Vec<_>>().into());
                accessor.set_element_type(ElementType::Scalar);

                primitive.set_indices(Some(accessor));
            }

            Some(mesh)
        }
        Err(_) => None,
    };

    node.set_mesh(mesh);
    node.set_name(name_to_string(name));
    node.set_translation(transform.translation.into());
    node.set_rotation(transform.rotation.into());
    node.set_scale(transform.scale.into());

    let children = match children {
        Some(children) => children.to_vec(),
        None => Vec::new(),
    };

    children
        .iter()
        .map(|ent| export_node(ent, gltf, nodes_query, meshes_query, meshes))
        .for_each(|mut child| node.add_child(&mut child));

    node
}

fn attribute_to_accessor(values: &VertexAttributeValues, gltf: &mut Gltf) -> Result<Accessor, ()> {
    let mut accessor = gltf.create_accessor();

    match values {
        VertexAttributeValues::Float32(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Scalar);
        }
        VertexAttributeValues::Float32x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
        }
        VertexAttributeValues::Float32x3(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec3);
        }
        VertexAttributeValues::Float32x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
        }
        VertexAttributeValues::Uint32(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Scalar);
        }
        VertexAttributeValues::Uint32x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
        }
        VertexAttributeValues::Uint32x3(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec3)
        }
        VertexAttributeValues::Uint32x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
        }
        VertexAttributeValues::Uint16x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
        }
        VertexAttributeValues::Uint16x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
        }
        VertexAttributeValues::Uint8x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
        }
        VertexAttributeValues::Uint8x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
        }
        VertexAttributeValues::Unorm16x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
            accessor.set_normalized(true);
        }
        VertexAttributeValues::Unorm16x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
            accessor.set_normalized(true);
        }
        VertexAttributeValues::Unorm8x4(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec4);
            accessor.set_normalized(true);
        }
        VertexAttributeValues::Unorm8x2(values) => {
            accessor.set_array(values.clone().into());
            accessor.set_element_type(ElementType::Vec2);
            accessor.set_normalized(true);
        }
        _ => {
            error!("Unsupported vertex attribute type");
            return Err(());
        }
    }

    Ok(accessor)
}
