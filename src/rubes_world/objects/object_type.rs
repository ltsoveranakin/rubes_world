use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub(crate) enum ObjectType {
    Cuboid(Vec3),
}

impl ObjectType {
    pub(super) fn get_collider_with_mesh(&self) -> (Collider, Mesh) {
        match self {
            Self::Cuboid(dimensions) => (
                Collider::cuboid(dimensions.x, dimensions.y, dimensions.z),
                Cuboid {
                    half_size: *dimensions,
                }
                .into(),
            ),
        }
    }
}
