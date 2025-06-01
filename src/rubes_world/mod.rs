mod camera;
mod objects;
mod ui;

use crate::rubes_world::camera::GameCameraPlugin;
use crate::rubes_world::objects::ObjectPlugin;
use crate::rubes_world::ui::GameUIPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(super) struct RubesWorldPlugin;

impl Plugin for RubesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_plugins((GameUIPlugin, ObjectPlugin, GameCameraPlugin));
    }
}
