use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub(crate) struct GameCamera;

fn spawn_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
        PanOrbitCamera::default(),
        IsDefaultUiCamera,
    ));

    ambient_light.brightness = 500.;
}
