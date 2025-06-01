use bevy::prelude::*;

pub(super) struct UIObjectSelectorPlugin;

impl Plugin for UIObjectSelectorPlugin {
    fn build(&self, app: &mut App) {}
}

pub(super) fn object_selector_ui() -> impl Bundle {
    (Node {
        width: Val::Percent(30.),
        height: Val::Percent(100.),
        ..default()
    },);
}
