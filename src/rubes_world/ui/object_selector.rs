use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::objects::{ModifySelectedObjectEvent, SelectedObject};
use crate::rubes_world::ui::checkbox::CheckBox;
use crate::rubes_world::ui::field::{field_check_box, field_text};
use crate::rubes_world::ui::{Parent, UIMouseBlock, UIRoot, UI_OVERLAY_COLOR};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_simple_text_input::TextInputValue;

pub(super) struct UIObjectSelectorPlugin;

impl Plugin for UIObjectSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_selector_ui.run_if(resource_changed::<SelectedObject>),
                set_object_properties,
                update_object_dynamic,
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct ObjectSelectorUI;

#[derive(Component)]
pub(super) struct FieldText {
    pub(super) label_name: String,
}

#[derive(Component)]
struct DynamicCheckBox;

pub(super) fn object_selector_ui(
    parent: Parent,
    object_type: &ObjectType,
    object_transform: &Transform,
) {
    parent
        .spawn((
            ObjectSelectorUI,
            Node {
                width: Val::Percent(30.),
                height: Val::Percent(100.),
                align_self: AlignSelf::End,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(UI_OVERLAY_COLOR),
            Name::new("UI Object Selector"),
            UIMouseBlock,
        ))
        .with_children(|parent| {
            parent.spawn(field_check_box("Dynamic", false, DynamicCheckBox));
            object_specific_ui(parent, object_type);
            transform_ui(parent, object_transform);
        });
}

fn object_specific_ui(parent: Parent, object_type: &ObjectType) {
    match object_type {
        ObjectType::Rectangle(dim) => {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    field_text("width", dim.x.to_string()),
                    field_text("height", dim.y.to_string()),
                ],
            ));
        }
        ObjectType::Circle(radius) => {
            parent.spawn((
                Node::default(),
                children![field_text("radius", radius.to_string()),],
            ));
        }
    }
}

fn transform_ui(parent: Parent, object_transform: &Transform) {
    info!("{}", object_transform.rotation);
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            field_text("x", object_transform.translation.x.to_string()),
            field_text("y", object_transform.translation.y.to_string()),
            field_text(
                "rotation",
                object_transform
                    .rotation
                    .to_euler(EulerRot::XYZ)
                    .2
                    .to_degrees()
                    .to_string()
            ),
        ],
    ));
}

fn set_object_properties(
    object_input_query: Query<(&FieldText, &TextInputValue), Changed<TextInputValue>>,
    mut selected_object_query: Query<(&mut ObjectType, &mut Transform)>,
    selected_object: Res<SelectedObject>,
    mut modify_selected_object_event: EventWriter<ModifySelectedObjectEvent>,
) {
    for (field_text, text_input_value) in object_input_query.iter() {
        if let Some(selected_object_entity) = selected_object.0 {
            let (mut selected_object_type, mut transform) = selected_object_query
                .get_mut(selected_object_entity)
                .unwrap();

            let input_parsed = text_input_value.0.parse().unwrap_or(0.);

            match &mut *selected_object_type {
                ObjectType::Rectangle(dim) => match &*field_text.label_name {
                    "width" => dim.x = input_parsed,
                    "height" => dim.y = input_parsed,
                    _ => {}
                },

                ObjectType::Circle(radius) => match &*field_text.label_name {
                    "radius" => *radius = input_parsed,
                    _ => {}
                },
            }

            match &*field_text.label_name {
                "x" => transform.translation.x = input_parsed,
                "y" => transform.translation.y = input_parsed,
                "rotation" => transform.rotation = Quat::from_rotation_z(input_parsed.to_radians()),
                _ => {}
            }

            info!("{}", transform.rotation);

            // info!("Obj type: {:?}", object_type);

            modify_selected_object_event.write(ModifySelectedObjectEvent);
        }
    }
}

fn update_selector_ui(
    mut commands: Commands,
    object_query: Query<(&ObjectType, &Transform)>,
    ui_root_query: Query<Entity, With<UIRoot>>,
    object_selector_ui_query: Query<Entity, With<ObjectSelectorUI>>,

    selected_object: Res<SelectedObject>,
) {
    if let Ok(object_selector_entity) = object_selector_ui_query.single() {
        commands.entity(object_selector_entity).despawn();
    }

    if let Some(selected_object_entity) = selected_object.0 {
        let ui_root_entity = ui_root_query.single().unwrap();
        let (object_type, object_transform) = object_query.get(selected_object_entity).unwrap();

        commands.entity(ui_root_entity).with_children(|parent| {
            object_selector_ui(parent, object_type, object_transform);
        });
    }
}

fn update_object_dynamic(
    mut commands: Commands,
    dynamic_check_box_query: Query<&CheckBox, (Changed<CheckBox>, With<DynamicCheckBox>)>,
    selected_object: Res<SelectedObject>,
) {
    for checkbox in dynamic_check_box_query.iter() {
        let mut commands = commands.entity(selected_object.0.unwrap());

        if checkbox.0 {
            commands.insert(RigidBody::Dynamic);
        } else {
            commands.insert(RigidBody::Fixed);
        }
    }
}
