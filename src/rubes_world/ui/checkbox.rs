use bevy::prelude::*;

pub(super) struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, checkbox_click);
    }
}

fn checkbox_border_color() -> BorderColor {
    Srgba::BLACK.into()
}

#[derive(Component)]
#[require(ImageNode, BorderColor = checkbox_border_color(), Button)]
pub(super) struct CheckBox(pub(super) bool);

fn checkbox_click(
    mut check_box_query: Query<
        (&mut CheckBox, &mut ImageNode, &Interaction),
        Or<(Changed<Interaction>, Changed<CheckBox>)>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (mut check_box, mut image_node, interaction) in check_box_query.iter_mut() {
        if interaction == &Interaction::Pressed {
            let check_value = check_box.0;
            check_box.0 = !check_value;
        }

        let img_name = if check_box.0 { "checkmark" } else { "x-icon" };

        image_node.image = asset_server.load(format!("images/ui/{}.png", img_name));
    }
}
