use crate::rubes_world::objects::object_type::ObjectType;
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct UIObject(pub(super) ObjectType);

pub(super) fn ui_object(
    image_handle: Handle<Image>,
    name: String,
    object_type: ObjectType,
) -> impl Bundle {
    (
        UIObject(object_type),
        Node {
            height: Val::Percent(100.),
            column_gap: Val::Percent(2.),
            flex_direction: FlexDirection::Column,
            aspect_ratio: Some(1.),
            ..default()
        },
        Name::new("UI Object"),
        Button::default(),
        Interaction::None,
        children![(ImageNode::new(image_handle),), Text::new(name)],
    )
}
