use bevy::prelude::*;

pub(super) struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, checkbox_click);
    }
}

fn checkbox_node() -> Node {
    Node {
        width: Val::Px(10.),
        height: Val::Px(10.),
        border: UiRect::all(Val::Px(4.)),
        ..default()
    }
}

fn checkbox_border_color() -> BorderColor {
    Srgba::BLACK.into()
}

fn checkbox_name() -> Name {
    Name::new("CheckBox")
}

#[derive(Component)]
#[require(Node = checkbox_node(), BorderColor = checkbox_border_color(), Name = checkbox_name(), Button)]
pub(super) struct CheckBox(pub(super) bool);

fn checkbox_click(
    mut check_box_query: Query<
        (&mut CheckBox, &mut BackgroundColor, &Interaction),
        Or<(Changed<Interaction>, Changed<CheckBox>)>,
    >,
) {
    for (mut check_box, mut background_color, interaction) in check_box_query.iter_mut() {
        if interaction == &Interaction::Pressed {
            let check_value = check_box.0;
            check_box.0 = !check_value;
        }
        background_color.0 = if check_box.0 {
            Srgba::BLACK
        } else {
            Srgba::WHITE
        }
        .into();
    }
}
