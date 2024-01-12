# bevy_gltf_kun

Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).

## Comparison

Compared to `bevy_gltf`, this plugin adds support for:
- Arbitrary glTF extensions
- glTF exporting
- [glXF](https://github.com/KhronosGroup/glXF) import and export

Additionally, the use of the `gltf_kun` graph format allows for easy use of other `gltf_kun` features
such as running transform functions to compress textures or simplify meshes.

## Overview

This plugin adds an asset type for `gltf_kun` documents, along with asset loaders for supported file formats.
**This makes it incompatible with `bevy_gltf`**, which also creates an asset loader for glTF files.
Bevy asset loaders claim file extensions, you can't have multiple loaders for the same file type.

An example of what this looks like in practice is the loading of a `.gltf` file into a `Handle<GltfDocument>` Bevy asset.
You can then interact with this document asset however you'd like, such as running `gltf_kun` transform functions on it.
When ready you can then consume the document asset and load it into the scene using an `Import<GltfDocument>` event.

Check out the examples to learn more.
