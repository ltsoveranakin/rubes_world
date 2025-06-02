mod object_selector;

mod checkbox;
mod toolbar;

use crate::rubes_world::ui::object_selector::{object_selector_ui, UIObjectSelectorPlugin};
use std::convert::Into;

use crate::rubes_world::ui::checkbox::CheckBoxPlugin;
use crate::rubes_world::ui::toolbar::{object_toolbar_ui, UIToolbarPlugin};
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIToolbarPlugin, UIObjectSelectorPlugin, CheckBoxPlugin))
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, ui_element_hovered);
    }
}

pub(super) const UI_OVERLAY_COLOR: Color = Color::srgba_u8(29, 34, 41, 100);

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Name::new("UI Root"),
        // BackgroundColor(Color::Srgba(Srgba::RED)),
        children![object_toolbar_ui(), object_selector_ui()],
    ));
    // commands.spawn(object_toolbar_ui());
}

fn ui_element_hovered(
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
