mod camera;
mod objects;
mod ui;

use crate::rubes_world::camera::GameCameraPlugin;
use crate::rubes_world::objects::ObjectPlugin;
use crate::rubes_world::ui::GameUIPlugin;
use bevy::prelude::*;

use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_rapier3d::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

pub(super) struct RubesWorldPlugin;

impl Plugin for RubesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins))
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                PanOrbitCameraPlugin,
                TextInputPlugin,
            ))
            .add_plugins((GameUIPlugin, ObjectPlugin, GameCameraPlugin));
    }
}
