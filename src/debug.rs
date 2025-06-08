use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                RapierDebugRenderPlugin::default(),
                EguiPlugin {
                    enable_multipass_for_primary_context: true,
                },
                WorldInspectorPlugin::new(),
            ));
        }
    }
}
