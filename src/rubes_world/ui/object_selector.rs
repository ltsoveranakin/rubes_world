use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::objects::{ModifySelectedObjectEvent, SelectedObject};
use crate::rubes_world::ui::checkbox::CheckBox;
use crate::rubes_world::ui::field::{field_check_box, field_text};
use crate::rubes_world::ui::{Parent, UIMouseBlock, UIRoot, UI_OVERLAY_COLOR};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_simple_text_input::TextInputValue;

pub(super) struct UIObjectSelectorPlugin;

impl Plugin for UIObjectSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_selector_ui.run_if(resource_changed::<SelectedObject>),
                set_object_specific_properties,
                update_object_dynamic,
            ),
        );
    }
}

#[derive(Component)]
pub(super) struct ObjectSelectorUI;

#[derive(Component)]
pub(super) struct ObjectInput {
    pub(super) label_name: String,
}

#[derive(Component)]
struct DynamicCheckBox;

pub(super) fn object_selector_ui(parent: Parent, object_type: ObjectType) {
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
            selector_content(parent, object_type);
        });
}

fn selector_content(parent: Parent, object_type: ObjectType) {
    parent.spawn(field_check_box("Dynamic", false, DynamicCheckBox));
    object_specific_ui(parent, object_type);
}

fn object_specific_ui(parent: Parent, object_type: ObjectType) {
    match object_type {
        ObjectType::Cuboid(dim) => {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    field_text("width", dim.x.to_string()),
                    field_text("height", dim.y.to_string()),
                    field_text("length", dim.z.to_string(),)
                ],
            ));
        }
        ObjectType::Sphere(radius) => {
            parent.spawn((
                Node::default(),
                children![field_text("radius", radius.to_string()),],
            ));
        }
    }
}

fn set_object_specific_properties(
    mut object_input_query: Query<(&mut ObjectInput, &TextInputValue), Changed<TextInputValue>>,
    mut selected_object_query: Query<&mut ObjectType>,
    selected_object: Res<SelectedObject>,
    mut modify_selected_object_event: EventWriter<ModifySelectedObjectEvent>,
) {
    for (object_input, text_input_value) in object_input_query.iter_mut() {
        if let Some(selected_object_entity) = selected_object.0 {
            let mut selected_object_type = selected_object_query
                .get_mut(selected_object_entity)
                .unwrap();

            let input_parsed = text_input_value.0.parse().unwrap_or(0.);

            match &mut *selected_object_type {
                ObjectType::Cuboid(dim) => match &*object_input.label_name {
                    "width" => dim.x = input_parsed,
                    "height" => dim.y = input_parsed,
                    "length" => dim.z = input_parsed,
                    _ => {}
                },

                ObjectType::Sphere(radius) => {
                    *radius = input_parsed;
                }
            }

            // info!("Obj type: {:?}", object_type);

            modify_selected_object_event.write(ModifySelectedObjectEvent);
        }
    }
}

fn update_selector_ui(
    mut commands: Commands,
    object_query: Query<&ObjectType>,
    ui_root_query: Query<Entity, With<UIRoot>>,
    object_selector_ui_query: Query<Entity, With<ObjectSelectorUI>>,

    selected_object: Res<SelectedObject>,
) {
    if let Ok(object_selector_entity) = object_selector_ui_query.single() {
        commands.entity(object_selector_entity).despawn();
    }

    if let Some(selected_object_entity) = selected_object.0 {
        let ui_root_entity = ui_root_query.single().unwrap();
        let object_type = *object_query.get(selected_object_entity).unwrap();

        commands.entity(ui_root_entity).with_children(|parent| {
            object_selector_ui(parent, object_type);
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
