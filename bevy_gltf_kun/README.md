# bevy_gltf_kun

Bevy glTF plugin using [gltf_kun](https://crates.io/crates/gltf_kun).

## Comparison

Compared to `bevy_gltf`, this plugin adds support for:
- Arbitrary glTF extensions
- Name-based asset labels
- glTF exporting
- [glXF](https://github.com/KhronosGroup/glXF) import and export

## Compatibility

Asset loading in Bevy requires claiming a file extension.
Thus, `.gltf` and `.glb` importing will only work if you do not have the `bevy_gltf` plugin added to your app.
This can be done by removing the `bevy_gltf` feature flag from Bevy (included in default-features).

In the future when Bevy allows you to choose an asset loader when loading a URI this will not be an issue.
