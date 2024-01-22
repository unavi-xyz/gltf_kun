use super::gltf::ExportContext;

pub trait BevyExtensionExport {
    fn bevy_export(&self, context: &mut ExportContext);
}
