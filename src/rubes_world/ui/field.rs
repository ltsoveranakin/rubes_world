use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::ui::checkbox::CheckBox;
use crate::rubes_world::ui::object_selector::ObjectInput;
use crate::rubes_world::ui::{BACKGROUND_COLOR, BORDER_COLOR_ACTIVE, TEXT_COLOR};
use bevy::prelude::*;
use bevy_simple_text_input::*;

pub(super) fn field_input(label_name: impl Into<String>, input_bundle: impl Bundle) -> impl Bundle {
    let label_name = label_name.into();
    (
        Node::default(),
        Name::new("Field"),
        children![
            (Text::new(label_name.clone()), Name::new("Field(Label)")),
            input_bundle
        ],
    )
}

pub(super) fn field_text(
    label_name: impl Into<String>,
    value: String,
    object_type: ObjectType,
) -> impl Bundle {
    let label_name = label_name.into();
    field_input(
        label_name.clone(),
        (
            Node {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(BORDER_COLOR_ACTIVE),
            BackgroundColor(BACKGROUND_COLOR),
            TextInput,
            TextInputTextFont(TextFont {
                font_size: 10.,
                ..default()
            }),
            TextInputTextColor(TextColor(TEXT_COLOR)),
            TextInputValue(value),
            TextInputInactive(true),
            TextInputSettings {
                retain_on_submit: true,
                ..default()
            },
            ObjectInput { label_name },
            Name::new("Field(Text Input)"),
        ),
    )
}

pub(super) fn field_check_box(label_name: impl Into<String>, value: bool) -> impl Bundle {
    field_input(label_name, CheckBox(value));
}
