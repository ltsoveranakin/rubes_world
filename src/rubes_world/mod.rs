mod camera;
mod objects;
mod ui;

use crate::rubes_world::camera::GameCameraPlugin;
use crate::rubes_world::objects::ObjectPlugin;
use crate::rubes_world::ui::GameUIPlugin;
use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

pub(super) struct RubesWorldPlugin;

impl Plugin for RubesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins))
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                TextInputPlugin,
            ))
            .add_plugins((GameUIPlugin, ObjectPlugin, GameCameraPlugin));
    }
}

// fn drag_log(mut drag_started: EventReader<GizmoDragStarted>) {
//     for drag in drag_started.read() {
//         info!("{:?}", drag);
//     }
// }
