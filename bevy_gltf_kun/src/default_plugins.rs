use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::GltfKunPlugin;

/// Copy of [bevy::DefaultPlugins], using `gltf_kun` instead of `bevy_gltf`.
///
/// Does not obey Cargo feature flags (I don't think it's possible to do so),
/// so all plugins are enabled.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin)
            .add(bevy::core::FrameCountPlugin)
            .add(bevy::time::TimePlugin)
            .add(bevy::transform::TransformPlugin)
            .add(bevy::hierarchy::HierarchyPlugin)
            .add(bevy::diagnostic::DiagnosticsPlugin)
            .add(bevy::input::InputPlugin)
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::a11y::AccessibilityPlugin)
            .add(bevy::asset::AssetPlugin::default())
            .add(bevy::scene::ScenePlugin)
            .add(bevy::winit::WinitPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::render::texture::ImagePlugin::default());

        #[cfg(not(target_family = "wasm"))]
        {
            group = group.add(bevy::render::pipelined_rendering::PipelinedRenderingPlugin);
        }

        group = group
            .add(bevy::core_pipeline::CorePipelinePlugin)
            .add(bevy::sprite::SpritePlugin)
            .add(bevy::text::TextPlugin)
            .add(bevy::ui::UiPlugin)
            .add(bevy::pbr::PbrPlugin::default())
            .add(GltfKunPlugin)
            .add(bevy::audio::AudioPlugin::default())
            .add(bevy::gilrs::GilrsPlugin)
            .add(bevy::animation::AnimationPlugin)
            .add(bevy::gizmos::GizmoPlugin);

        group
    }
}
