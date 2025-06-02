use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Component, Copy, Clone)]
pub(crate) enum ObjectType {
    Cuboid(Vec3),
}

impl ObjectType {
    pub(super) fn get_collider(&self) -> Collider {
        match self {
            Self::Cuboid(dim) => Collider::cuboid(dim.x, dim.y, dim.z),
        }
    }

    pub(super) fn get_mesh(&self) -> Mesh {
        match self {
            Self::Cuboid(dimensions) => Cuboid {
                half_size: *dimensions,
            }
            .into(),
        }
    }
}
