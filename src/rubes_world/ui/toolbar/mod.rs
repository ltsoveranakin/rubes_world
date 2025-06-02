mod object;

use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::objects::SpawnObjectEvent;
use crate::rubes_world::ui::toolbar::object::{ui_object, UIObject};
use crate::rubes_world::ui::UI_OVERLAY_COLOR;
use bevy::prelude::*;

pub(super) struct UIToolbarPlugin;

impl Plugin for UIToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateUIObjectEvent>()
            .add_systems(Startup, spawn_default_ui_objects)
            .add_systems(Update, (listen_spawn_new_ui_object, click_ui_object));
    }
}

#[derive(Event)]
struct CreateUIObjectEvent {
    name: String,
}

#[derive(Component)]
struct UIToolbar;

pub(super) fn object_toolbar_ui() -> impl Bundle {
    (
        UIToolbar,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Name::new("UI Toolbar"),
        BackgroundColor(UI_OVERLAY_COLOR),
    )
}

fn spawn_default_ui_objects(mut create_ui_object_event: EventWriter<CreateUIObjectEvent>) {
    create_ui_object_event.write(CreateUIObjectEvent {
        name: "Cuboid".into(),
    });
}

fn listen_spawn_new_ui_object(
    mut commands: Commands,
    toolbar_query: Query<Entity, With<UIToolbar>>,
    asset_server: Res<AssetServer>,
    mut create_ui_object_event: EventReader<CreateUIObjectEvent>,
) {
    for create_ui_object in create_ui_object_event.read() {
        let toolbar_entity = toolbar_query.single().unwrap();
        commands.entity(toolbar_entity).insert(children![ui_object(
            asset_server.load("cuboid_placeholder.png"),
            create_ui_object.name.clone(),
        )]);
    }
}

fn click_ui_object(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<UIObject>)>,
    mut spawn_object_event: EventWriter<SpawnObjectEvent>,
) {
    for interaction in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            spawn_object_event.write(SpawnObjectEvent {
                object_type: ObjectType::Cuboid(Vec3::splat(0.5)),
            });
        }
    }
}
