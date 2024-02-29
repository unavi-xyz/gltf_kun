use bevy::prelude::*;

use super::ExportContext;

pub fn export_skins(In(mut context): In<ExportContext>) -> ExportContext {
    context
}
