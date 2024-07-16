# bevy_gltf_kun

<!-- cargo-rdme start -->

Bevy [gltf_kun](https://crates.io/crates/gltf_kun) plugin.

### Features

- glTF extension processing via hooks
- [glTF](https://github.com/KhronosGroup/glTF) import / export
- [glXF](https://github.com/KhronosGroup/glXF) import / export

### Usage

Add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_gltf_kun::GltfKunPlugin;

let app = App::new().add_plugins((DefaultPlugins, GltfKunPlugin::default()));
```

#### Export

Export scenes to glTF using [GltfExportEvent](export::gltf::GltfExportEvent).

The resulting [GltfExportResult](export::gltf::GltfExportResult) will contain a flexible
[GltfDocument](gltf_kun::graph::gltf::document::GltfDocument) that can be exported to various
file types. See [gltf_kun] for more information on how to do so.


```rust
use bevy::prelude::*;
use bevy_gltf_kun::{
    extensions::DefaultExtensions,
    export::gltf::{GltfExportEvent, GltfExportResult}
};
use gltf_kun::io::format::glb::GlbExport;

fn export_scene(
    scenes: Query<&Handle<Scene>>,
    mut export: EventWriter<GltfExportEvent<DefaultExtensions>>,
    mut results: ResMut<Events<GltfExportResult>>,
    mut did_export: Local<bool>,
) {
    // Send an export event once.
    if !*did_export {
        if let Some(handle) = scenes.iter().next() {
            export.send(GltfExportEvent::new(handle.clone()));
            *did_export = true;
        }
    }

    // Listen for the result.
    for mut event in results.drain() {
        let doc = event.result.unwrap();
        let bytes = GlbExport::<DefaultExtensions>::export(&mut event.graph, &doc);
    }
}

App::new().add_systems(Update, export_scene);
```

#### Import

Import glTFs using the [GltfKun](import::gltf::GltfKun) asset.

```rust
use bevy::prelude::*;
use bevy_gltf_kun::import::gltf::{GltfKun, scene::GltfScene};

fn import_gltf(
    asset_server: Res<AssetServer>,
    gltf_kun_assets: Res<Assets<GltfKun>>,
    gltf_scene_assets: Res<Assets<GltfScene>>,
    mut commands: Commands,
    mut handle: Local<Option<Handle<GltfKun>>>,
    mut did_import: Local<bool>,
) {
    if *did_import {
        return;
    }

    // Load the asset.
    if handle.is_none() {
        *handle = Some(asset_server.load::<GltfKun>("model.gltf"));
    }

    let handle = handle.as_ref().unwrap();

    let gltf = match gltf_kun_assets.get(handle) {
        Some(a) => a,
        None => return,
    };

    // Spawn the first scene.
    let gltf_scene = gltf_scene_assets.get(&gltf.scenes[0]).unwrap();
    commands.spawn(SceneBundle { scene: gltf_scene.scene.clone(), ..default() });

    *did_import = true;
}

App::new().add_systems(Update, import_gltf);
```

<!-- cargo-rdme end -->
