use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::objects::SelectedObject;
use crate::rubes_world::ui::checkbox::CheckBox;
use crate::rubes_world::ui::UI_OVERLAY_COLOR;
use bevy::prelude::*;

pub(super) struct UIObjectSelectorPlugin;

impl Plugin for UIObjectSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_selector_ui.run_if(resource_changed::<SelectedObject>),
        );
    }
}

#[derive(Component)]
struct ObjectSelectorUI;

pub(super) fn object_selector_ui() -> impl Bundle {
    (
        ObjectSelectorUI,
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(100.),
            align_self: AlignSelf::End,
            // display: Display::None,
            ..default()
        },
        Name::new("UI Object Selector"),
        BackgroundColor(UI_OVERLAY_COLOR),
        children![CheckBox(false)],
    )
}

fn update_selector_ui(
    mut commands: Commands,
    mut object_selector_ui_query: Query<&mut Node, With<ObjectSelectorUI>>,
    selected_object_query: Query<&ObjectType>,
    selected_object: Res<SelectedObject>,
) {
    let mut object_selector_node = object_selector_ui_query.single_mut().unwrap();
    if let Some(selected_object_entity) = selected_object.0 {
        object_selector_node.display = Display::Flex;
    } else {
        object_selector_node.display = Display::None;
    }
}
