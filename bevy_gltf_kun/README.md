# bevy_gltf_kun

Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).

Compared to `bevy_gltf`, this plugin adds support for:
- Arbitrary glTF extensions
- glTF exporting
- [glXF](https://github.com/KhronosGroup/glXF) import and export

Additionally, the use of the `gltf_kun` graph format allows for easy use of other `gltf_kun` features
such as running transform functions on your models to compress textures or simplify meshes.
