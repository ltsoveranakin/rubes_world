pub(crate) mod object_type;

use crate::rubes_world::objects::object_type::ObjectType;
use bevy::prelude::*;
use bevy_rapier3d::na::DimAdd;
use bevy_rapier3d::prelude::*;

pub(super) struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnObjectEvent>()
            .add_systems(Update, listen_spawn_object);
    }
}

#[derive(Event)]
pub(crate) struct SpawnObjectEvent {
    pub(crate) object_type: ObjectType,
}

fn listen_spawn_object(
    mut commands: Commands,
    mut spawn_object_event: EventReader<SpawnObjectEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
