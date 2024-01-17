use std::error::Error;

use crate::{document::GltfDocument, io::format::gltf::GltfFormat};

use super::{ExtensionIO, OMIPhysicsBodyExtension, EXTENSION_NAME};

impl ExtensionIO<GltfDocument, GltfFormat> for OMIPhysicsBodyExtension {
    fn name(&self) -> &'static str {
        EXTENSION_NAME
    }

    fn export(
        &self,
        _doc: &mut GltfDocument,
        _format: &mut GltfFormat,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn import(
        &self,
        _format: &mut GltfFormat,
        _doc: &mut GltfDocument,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extensions::omi_physics_body::{
            physics_body::{BodyType, Motion},
            OMIPhysicsBodyExtension,
        },
        io::format::glb::GlbIO,
    };

    use super::*;

    #[test]
    fn test_io() {
        let mut doc = GltfDocument::default();
        let node = doc.create_node();

        let mut physics_body = OMIPhysicsBodyExtension::create_body(&mut doc.0, &node);

        let mut weight = physics_body.get(&doc.0);
        weight.motion = Some(Motion::new(BodyType::Dynamic));

        physics_body.set(&mut doc.0, weight);

        let mut io = GlbIO::default();
        io.extensions.add(OMIPhysicsBodyExtension);
    }
}
