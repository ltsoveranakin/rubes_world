mod object_selector;

mod toolbar;

use crate::rubes_world::ui::object_selector::UIObjectSelectorPlugin;

use crate::rubes_world::ui::toolbar::{object_toolbar_ui, UIToolbarPlugin};
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIToolbarPlugin, UIObjectSelectorPlugin))
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, ui_element_hovered);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // BackgroundColor(Color::Srgba(Srgba::RED)),
        children![object_toolbar_ui()],
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
