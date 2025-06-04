use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Component, Reflect, Copy, Clone, Debug)]
pub(crate) enum ObjectType {
    Cuboid(Vec3),
}

impl Default for ObjectType {
    fn default() -> Self {
        Self::Cuboid(Vec3::splat(0.5))
    }
}

impl ObjectType {
    pub(super) fn get_collider(&self) -> Collider {
        match self {
            Self::Cuboid(dim) => Collider::cuboid(dim.x, dim.y, dim.z),
        }
    }

    pub(super) fn get_mesh(&self) -> Mesh {
        match self {
            Self::Cuboid(dim) => Cuboid { half_size: *dim }.into(),
        }
    }
}
