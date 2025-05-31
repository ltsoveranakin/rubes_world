mod rubes_world;

use crate::rubes_world::RubesWorldPlugin;
use bevy::prelude::App;

fn main() {
    App::new().add_plugins(RubesWorldPlugin).run();
}
