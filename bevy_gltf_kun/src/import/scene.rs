use bevy::prelude::*;

use super::ImportContext;

pub fn import_scenes(In(mut context): In<ImportContext>, mut commands: Commands) -> ImportContext {
    context.doc.scenes().iter().for_each(|scene| {
        let mut world = World::default();

        world.spawn(SpatialBundle::INHERITED_IDENTITY).id();

        let handle = Scene::new(world);
    });

    context
}
