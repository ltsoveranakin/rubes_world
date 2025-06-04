mod object_selector;

mod checkbox;
mod field;
mod toolbar;

use crate::rubes_world::ui::checkbox::CheckBoxPlugin;
use crate::rubes_world::ui::object_selector::{ObjectSelectorUI, UIObjectSelectorPlugin};
use crate::rubes_world::ui::toolbar::{object_toolbar_ui, UIToolbarPlugin};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;
use bevy_simple_text_input::{TextInput, TextInputInactive};
use std::convert::Into;

pub(super) const UI_OVERLAY_COLOR: Color = Color::srgba_u8(29, 34, 41, 100);
const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
pub(super) const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub(super) const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIToolbarPlugin, UIObjectSelectorPlugin, CheckBoxPlugin))
            .init_resource::<MouseOverBlockingUI>()
            .init_resource::<FocusedTextInput>()
            .add_event::<MouseBlockSafeEvent>()
            .add_systems(Startup, spawn_ui)
            .add_systems(
                Update,
                (
                    ui_interaction_hovered,
                    ui_mouse_move,
                    ui_block_mouse_click,
                    text_input_clicked,
                    text_input_focus_change.run_if(resource_changed::<FocusedTextInput>),
                ),
            );
    }
}

pub(super) type Parent<'a, 'w> = &'a mut RelatedSpawnerCommands<'w, ChildOf>;

#[derive(Component)]
#[require(Interaction::None)]
struct UIMouseBlock;

#[derive(Component)]
struct UIRoot;

#[derive(Resource, Default)]
struct MouseOverBlockingUI(bool);

#[derive(Resource, Default)]
struct FocusedTextInput {
    old: Option<Entity>,
    current: Option<Entity>,
}

impl FocusedTextInput {
    fn set_current(&mut self, input_entity: Option<Entity>) {
        self.old = self.current;
        self.current = input_entity;
    }
}

#[derive(Event)]
pub(crate) struct MouseBlockSafeEvent(pub(crate) MouseButtonInput);

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        UIRoot,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Name::new("UI Root"),
        // BackgroundColor(Color::Srgba(Srgba::RED)),
        children![object_toolbar_ui(), ObjectSelectorUI],
    ));
    // commands.spawn(object_toolbar_ui());
}

fn ui_interaction_hovered(
    mut commands: Commands,
    window_query: Query<Entity, With<Window>>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    let mut changed = false;
    let mut new_cursor = CursorIcon::System(SystemCursorIcon::Default);
    for interaction in interaction_query.iter() {
        changed = true;
        if interaction == &Interaction::Hovered {
            new_cursor = CursorIcon::System(SystemCursorIcon::Pointer)
        }
    }

    if changed {
        let window_entity = window_query.single().unwrap();
        commands.entity(window_entity).insert(new_cursor);
    }
}

fn ui_mouse_move(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<UIMouseBlock>)>,
    mut mouse_over_blocking_ui: ResMut<MouseOverBlockingUI>,
) {
    for interaction in interaction_query.iter() {
        if interaction == &Interaction::Hovered {
            mouse_over_blocking_ui.0 = true;
            return;
        }
    }

    mouse_over_blocking_ui.0 = false;
}

fn ui_block_mouse_click(
    mouse_over_blocking_ui: Res<MouseOverBlockingUI>,
    mut mouse_block_safe_event: EventWriter<MouseBlockSafeEvent>,
    mut mouse_input_event: EventReader<MouseButtonInput>,
) {
    for mouse_input in mouse_input_event.read() {
        if !mouse_over_blocking_ui.0 {
            mouse_block_safe_event.write(MouseBlockSafeEvent(*mouse_input));
        }
    }
}

fn text_input_clicked(
    mut text_input_query: Query<(Entity, &Interaction), With<TextInput>>,
    mut focused_text_input: ResMut<FocusedTextInput>,
) {
    for (entity, interaction) in text_input_query.iter_mut() {
        if interaction == &Interaction::Pressed {
            focused_text_input.set_current(Some(entity));
        }
    }
}

fn text_input_focus_change(
    mut text_input_query: Query<&mut TextInputInactive>,
    focused_text_input: Res<FocusedTextInput>,
) {
    if focused_text_input.old == focused_text_input.current {
        return;
    }

    if let Some(old_focused_entity) = focused_text_input.old {
        if let Ok(mut inactive) = text_input_query.get_mut(old_focused_entity) {
            inactive.0 = true;
        }
    }

    if let Some(focused_entity) = focused_text_input.current {
        if let Ok(mut inactive) = text_input_query.get_mut(focused_entity) {
            inactive.0 = false;
        }
    }
}
