use bevy::prelude::*;

fn export_mesh(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    mesh_assets: &Res<Assets<Mesh>>,
    meshes: &Query<&Handle<Mesh>>,
    names: &Query<&Name>,
    child_ents: &BTreeMap<node::Node, Entity>,
    entity: Entity,
) -> Option<mesh::Mesh> {
    // Bevy meshes roughly correspond to glTF primitives,
    // so we need to find valid Bevy meshes to add as
    // primitives to our glTF mesh.
    let mut primitive_ents = Vec::new();

    if meshes.contains(entity) {
        primitive_ents.push(entity);
    }

    child_ents.iter().for_each(|(node, ent)| {
        // Valid child nodes have no children of their own.
        if !node.children(graph).is_empty() {
            return;
        }

        // Valid child nodes have no transform.
        let weight = node.get(graph);
        if weight.translation != glam::Vec3::ZERO
            || weight.rotation != glam::Quat::IDENTITY
            || weight.scale != glam::Vec3::ONE
        {
            return;
        }

        if meshes.contains(*ent) {
            primitive_ents.push(*ent);
        }
    });

    if primitive_ents.is_empty() {
        return None;
    }

    let bevy_meshes = primitive_ents
        .iter()
        .map(|ent| meshes.get(*ent).unwrap().clone())
        .collect::<Vec<_>>();

    // Check cache for existing glTF mesh using the same Bevy meshes.
    if let Some(cached) = context.meshes.iter().find(|cached| {
        bevy_meshes.len() == cached.bevy_meshes.len()
            && bevy_meshes
                .iter()
                .all(|mesh| cached.bevy_meshes.contains(mesh))
    }) {
        return Some(cached.mesh);
    }

    // Create new mesh.
    let mut mesh = mesh::Mesh::new(graph);
    let weight = mesh.get_mut(graph);

    if let Ok(name) = names.get(entity) {
        weight.name = Some(name.to_string());
    }

    primitive_ents.iter().for_each(|ent| {
        let handle = meshes.get(*ent).unwrap();
        let bevy_mesh = match mesh_assets.get(handle) {
            Some(mesh) => mesh,
            None => {
                error!("Mesh not found: {:?}", handle);
                return;
            }
        };

        match export_primitive(graph, context, bevy_mesh) {
            Ok(primitive) => mesh.add_primitive(graph, &primitive),
            Err(e) => {
                error!("Error exporting primitive: {}", e);
            }
        }
    });

    context.meshes.push(CachedMesh { mesh, bevy_meshes });

    Some(mesh)
}

fn export_primitive(
    graph: &mut GltfGraph,
    context: &mut ExportContext,
    mesh: &Mesh,
) -> Result<primitive::Primitive> {
    let mut primitive = primitive::Primitive::new(graph);
    let weight = primitive.get_mut(graph);

    Ok(primitive)
}
