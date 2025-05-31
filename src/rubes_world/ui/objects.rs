use crate::rubes_world::objects::object_type::ObjectType;
use crate::rubes_world::objects::SpawnObjectEvent;
use bevy::prelude::*;

pub(super) struct UIObjectPlugin;

impl Plugin for UIObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateUIObjectEvent>()
            .add_systems(Startup, spawn_default_ui_objects)
            .add_systems(Update, (listen_create_ui_object, click_ui_object));
    }
}

#[derive(Event)]
struct CreateUIObjectEvent {
    name: String,
}

#[derive(Component)]
struct UIObject;

fn spawn_default_ui_objects(mut create_ui_object_event: EventWriter<CreateUIObjectEvent>) {
    create_ui_object_event.write(CreateUIObjectEvent {
        name: "Cuboid".into(),
    });
}

fn listen_create_ui_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut create_ui_object_event: EventReader<CreateUIObjectEvent>,
) {
    for create_ui_object in create_ui_object_event.read() {
        commands.spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(10.),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(23, 28, 41)),
            children![(
                UIObject,
                Node {
                    width: Val::Percent(10.),
                    max_height: Val::Percent(100.),
                    padding: UiRect::bottom(Val::Percent(50.)),
                    flex_direction: FlexDirection::Column,
                    aspect_ratio: Some(1.),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(43, 51, 71)),
                Button::default(),
                Interaction::None,
                children![
                    (ImageNode::new(asset_server.load("cuboid_placeholder.png")),),
                    Text::new(&create_ui_object.name)
                ]
            )],
        ));
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
