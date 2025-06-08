use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Reflect, Copy, Clone, Debug)]
pub(crate) enum ObjectType {
    Rectangle(Vec2),
    Circle(f32),
}

impl Default for ObjectType {
    fn default() -> Self {
        Self::Rectangle(Vec2::splat(5.))
    }
}

impl ObjectType {
    pub(super) fn get_collider(&self) -> Collider {
        match self {
            Self::Rectangle(dim) => Collider::cuboid(dim.x, dim.y),
            Self::Circle(radius) => Collider::ball(*radius),
        }
    }

    pub(super) fn get_mesh(&self) -> Mesh {
        match self {
            Self::Rectangle(dim) => Rectangle { half_size: *dim }.into(),
            Self::Circle(radius) => Circle { radius: *radius }.into(),
        }
    }
}
