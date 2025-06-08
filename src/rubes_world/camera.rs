use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, scroll_camera);
    }
}

#[derive(Component)]
pub(crate) struct GameCamera;

fn spawn_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0., 0., 0.),
        GameCamera,
        IsDefaultUiCamera,
    ));

    ambient_light.brightness = 500.;
}

fn scroll_camera(
    mut camera_query: Query<&mut Projection, With<GameCamera>>,
    mut mouse_wheel_event: EventReader<MouseWheel>,
) {
    for mouse_wheel in mouse_wheel_event.read() {
        let mut projection = camera_query.single_mut().unwrap();
        match &mut *projection {
            Projection::Orthographic(ortho) => {
                ortho.scale += mouse_wheel.y / -100.;
                ortho.scale = ortho.scale.clamp(0.05, 5.);
            }

            _ => {}
        }
    }
}
