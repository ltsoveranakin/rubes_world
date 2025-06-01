pub(crate) mod object_type;

use crate::rubes_world::objects::object_type::ObjectType;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::rubes_world::camera::GameCamera;
use bevy_rapier3d::prelude::*;

pub(super) struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnObjectEvent>()
            .add_systems(Update, (listen_spawn_object, left_click_sel_object));
    }
}

#[derive(Event)]
pub(crate) struct SpawnObjectEvent {
    pub(crate) object_type: ObjectType,
}

fn listen_spawn_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_object_event: EventReader<SpawnObjectEvent>,
) {
    for spawn_object in spawn_object_event.read() {
        let (collider, mesh) = spawn_object.object_type.get_collider_with_mesh();
        commands.spawn((
            collider,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(Color::srgb_u8(227, 46, 14))),
        ));
    }
}

fn left_click_sel_object(
    rapier_context: ReadRapierContext,
    camera_query: Query<&mut Transform, With<GameCamera>>,
    mut mouse_input_event: EventReader<MouseButtonInput>,
) {
    for mouse_input in mouse_input_event.read() {
        if mouse_input.button == MouseButton::Left && mouse_input.state.is_pressed() {
            let camera_transform = camera_query.single().unwrap();

            let ray_pos = camera_transform.translation;
            let ray_dir = camera_transform.rotation * -Vec3::Z;
            let max_toi = f32::MAX;
            let solid = true;
            let filter = QueryFilter::default();

            let ctx = rapier_context.single().unwrap();

            if let Some((entity, _)) =
                ctx.cast_ray(ray_pos, ray_dir.into(), max_toi.into(), solid, filter)
            {}
        }
    }
}
