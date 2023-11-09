use bevy::asset::AssetId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use gltf_kun::json;
use gltf_kun::json::scene::UnitQuaternion;
use json::validation::Checked::Valid;
use std::borrow::Cow;
use std::mem;

#[derive(Default)]
struct ExportMaps {
    material: HashMap<AssetId<StandardMaterial>, json::Index<json::Material>>,
    mesh: HashMap<AssetId<Mesh>, json::Index<json::Mesh>>,
}

struct ExportContext {
    buffer: json::Buffer,
    materials: Vec<json::Material>,
    meshes: Vec<json::Mesh>,
    nodes: Vec<json::Node>,
    map: ExportMaps,
}

impl Default for ExportContext {
    fn default() -> Self {
        Self {
            buffer: json::Buffer {
                byte_length: 0,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                uri: None,
            },
            materials: Vec::new(),
            meshes: Vec::new(),
            nodes: Vec::new(),
            map: ExportMaps::default(),
        }
    }
}

pub fn export_gltf(
    mut events: EventReader<super::ExportScene>,
    meshes: Res<Assets<Mesh>>,
    scenes: Query<(Option<&Name>, Option<&Children>), With<Handle<Scene>>>,
    nodes_query: Query<(&Transform, Option<&Name>, Option<&Children>)>,
    meshes_query: Query<(&Handle<Mesh>, Option<&Name>)>,
) {
    for event in events.read() {
        let mut context = ExportContext::default();

        let (name, children) = scenes.get(event.scene).unwrap();

        let name = match name {
            Some(name) => Some(name.to_string()),
            _ => None,
        };

        let nodes = match children {
            Some(children) => children
                .iter()
                .map(|ent| export_node(ent, &mut context, &nodes_query, &meshes_query))
                .collect(),
            _ => vec![],
        };

        let scene = json::Scene {
            nodes,
            name,
            extras: default(),
            extensions: default(),
        };

        let root = json::Root {
            asset: json::Asset {
                generator: "bevy_gltf_kun".to_string().into(),
                ..default()
            },
            materials: context.materials,
            meshes: context.meshes,
            nodes: context.nodes,
            scenes: vec![scene],
            ..default()
        };

        let buffer_length = 0;

        match event.format {
            super::ExportFormat::Standard => {
                context.buffer.uri = Some("buffer.bin".to_string());

                if let Ok(out) = json::serialize::to_vec_pretty(&root) {
                    info!("Exported scene as GLTF. Size: {} bytes", out.len());
                } else {
                    error!("Failed to export scene as GLTF");
                };

                // let bin = to_padded_byte_vector(triangle_vertices);
            }

            super::ExportFormat::Binary => {
                let json_string = json::serialize::to_string(&root).expect("Serialization error");
                let mut json_offset = json_string.len() as u32;
                align_to_multiple_of_four(&mut json_offset);

                let glb = gltf_kun::binary::Glb {
                    header: gltf_kun::binary::Header {
                        magic: *b"glTF",
                        version: 2,
                        length: json_offset + buffer_length,
                    },
                    bin: None,
                    // bin: Some(Cow::Owned(to_padded_byte_vector(vertices))),
                    json: Cow::Owned(json_string.into_bytes()),
                };

                if let Ok(out) = glb.to_vec() {
                    info!("Exported scene as GLB. Size: {} bytes", out.len());
                } else {
                    error!("Failed to export scene as GLB");
                };
            }
        }
    }
}

fn export_node(
    entity: &Entity,
    context: &mut ExportContext,
    nodes: &Query<(&Transform, Option<&Name>, Option<&Children>)>,
    meshes: &Query<(&Handle<Mesh>, Option<&Name>)>,
) -> json::Index<json::Node> {
    let (transform, name, children) = nodes.get(*entity).unwrap();

    let name = match name {
        Some(name) => Some(name.as_str().into()),
        _ => None,
    };

    // Find mesh primitives
    // A mesh primitive is an entity with a Mesh that is either:
    // - the current node
    // - a child of the current node and has no children of its own.
    let mut primitive_children = Vec::new();
    let mut primitives = match children {
        Some(children) => children
            .iter()
            .filter_map(|child| {
                let mesh = match meshes.get(*child) {
                    Ok(mesh) => mesh,
                    _ => return None,
                };

                match nodes.get(*child) {
                    Ok((_, _, Some(grand_children))) => match grand_children.len() {
                        0 => (),
                        _ => return None,
                    },
                    _ => (),
                };

                primitive_children.push(child);

                Some(mesh)
            })
            .collect(),
        None => Vec::new(),
    };

    if let Ok(mesh) = meshes.get(*entity) {
        primitives.push(mesh);
    }

    let mesh = export_mesh(&primitives, context, meshes);

    let children = match children {
        Some(children) => Some(
            children
                .iter()
                .filter(|child| !primitive_children.contains(child))
                .map(|ent| export_node(ent, context, nodes, meshes))
                .collect(),
        ),
        _ => None,
    };

    let node = json::Node {
        camera: None,
        children,
        extensions: default(),
        extras: default(),
        matrix: None,
        mesh: None,
        name,
        rotation: Some(UnitQuaternion(transform.rotation.into())),
        scale: Some(transform.scale.into()),
        translation: Some(transform.translation.into()),
        skin: None,
        weights: None,
    };

    context.nodes.push(node);

    json::Index::new(context.nodes.len() as u32 - 1)
}

fn export_mesh(
    primitives: &Vec<(&Handle<Mesh>, Option<&Name>)>,
    context: &mut ExportContext,
    meshes: &Query<(&Handle<Mesh>, Option<&Name>)>,
) -> json::Index<json::Mesh> {
    let primitives = primitives
        .iter()
        .map(|(mesh, name)| {
            json::mesh::Primitive {
                extras: default(),
                extensions: default(),
                targets: None,
                mode: Valid(json::mesh::Mode::Triangles),
                indices: None,
                material: None,
                attributes: {
                    let mut map = std::collections::BTreeMap::new();
                    // map.insert(Valid(json::mesh::Semantic::Positions), json::Index::new(0));
                    // map.insert(Valid(json::mesh::Semantic::Colors(0)), json::Index::new(1));
                    map
                },
            }
        })
        .collect();

    let mesh = json::Mesh {
        name: None,
        primitives,
        weights: None,
        extras: default(),
        extensions: default(),
    };

    context.meshes.push(mesh);

    let index = json::Index::new(context.meshes.len() as u32 - 1);

    index
}

fn align_to_multiple_of_four(n: &mut u32) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}
