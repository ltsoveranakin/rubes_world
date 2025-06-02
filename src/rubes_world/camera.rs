use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, mouse_moved);
    }
}

#[derive(Component)]
pub(crate) struct GameCamera;

fn spawn_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
        IsDefaultUiCamera,
    ));

    ambient_light.brightness = 500.;
}

fn mouse_moved(
    mut camera_transform_query: Query<&mut Transform, With<GameCamera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_event: EventReader<MouseMotion>,
) {
    for mouse_motion in mouse_motion_event.read() {
        let mut camera_transform = camera_transform_query.single_mut().unwrap();
        if mouse_input.pressed(MouseButton::Middle) {
            camera_transform.rotate_y(mouse_motion.delta.y * 0.01);
        }
    }
}
