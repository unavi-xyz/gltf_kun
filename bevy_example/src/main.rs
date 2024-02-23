use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_example::ExamplePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_plugins((DefaultPlugins, ExamplePlugin))
        .run();
}
