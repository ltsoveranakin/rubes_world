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
            .init_resource::<SelectedObject>()
            .add_systems(Update, (listen_spawn_object, left_click_sel_object));
    }
}

#[derive(Event)]
pub(crate) struct SpawnObjectEvent {
    pub(crate) object_type: ObjectType,
}

#[derive(Resource, Default, Deref)]
pub(crate) struct SelectedObject(pub(crate) Option<Entity>);

fn listen_spawn_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_object_event: EventReader<SpawnObjectEvent>,
) {
    for spawn_object in spawn_object_event.read() {
        let object_type = &spawn_object.object_type;
        let collider = object_type.get_collider();
        let mesh = object_type.get_mesh();

        commands.spawn((
            collider,
            spawn_object.object_type,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(Color::srgb_u8(227, 46, 14))),
        ));
    }
}

fn left_click_sel_object(
    rapier_context: ReadRapierContext,
    camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), With<GameCamera>>,
    window_query: Query<&Window>,
    mut selected_object: ResMut<SelectedObject>,
    mut mouse_input_event: EventReader<MouseButtonInput>,
) {
    for mouse_input in mouse_input_event.read() {
        if mouse_input.button == MouseButton::Right && mouse_input.state.is_pressed() {
            let (camera, camera_transform, camera_global_transform) =
                camera_query.single().unwrap();
            let window = window_query.single().unwrap();

            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(ray) = camera.viewport_to_world(camera_global_transform, cursor_position)
                {
                    let ray_pos = ray.origin;
                    let ray_dir = ray.direction.into();
                    let max_toi = f32::MAX.into();
                    let solid = true;
                    let filter = QueryFilter::default();

                    let ctx = rapier_context.single().unwrap();

                    selected_object.0 = if let Some((entity, _)) =
                        ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                    {
                        info!("select: {}", entity);
                        Some(entity)
                    } else {
                        info!("Miss entity");
                        None
                    }
                }
            }
        }
    }
}
