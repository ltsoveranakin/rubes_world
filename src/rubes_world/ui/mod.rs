mod objects;

use crate::rubes_world::ui::objects::UIObjectPlugin;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

pub(super) struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIObjectPlugin)
            .add_systems(Update, ui_element_hovered);
    }
}

fn ui_element_hovered(
    mut commands: Commands,
    window_query: Query<Entity, With<Window>>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    let mut changed = false;
    let mut changed_to = CursorIcon::System(SystemCursorIcon::Default);
    for interaction in interaction_query.iter() {
        changed = true;
        if interaction == &Interaction::Hovered {
            changed_to = CursorIcon::System(SystemCursorIcon::Pointer)
        }
    }

    if changed {
        let window_entity = window_query.single().unwrap();
        commands.entity(window_entity).insert(changed_to);
    }
}
