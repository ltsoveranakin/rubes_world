mod objects;
mod ui;

use bevy::prelude::*;

use crate::rubes_world::objects::ObjectPlugin;
use crate::rubes_world::ui::GameUIPlugin;
use bevy_rapier3d::prelude::*;

pub(super) struct RubesWorldPlugin;

impl Plugin for RubesWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_plugins((GameUIPlugin, ObjectPlugin))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    ambient_light.brightness = 500.;
}
