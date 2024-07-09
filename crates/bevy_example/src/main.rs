use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_example::ExamplePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            ExamplePlugin,
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.2)))
        .run();
}
