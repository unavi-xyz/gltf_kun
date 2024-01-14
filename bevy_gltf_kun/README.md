# bevy_gltf_kun

Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).

## Comparison

Compared to `bevy_gltf`, this plugin adds support for:
- Arbitrary glTF extensions
- Name-based asset labels
- glTF exporting
- [glXF](https://github.com/KhronosGroup/glXF) import and export

Additionally, the use of the `gltf_kun` graph format allows for easy use of other `gltf_kun` features
such as running transform functions to compress textures or simplify meshes.
