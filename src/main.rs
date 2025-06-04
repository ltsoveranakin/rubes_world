mod debug;
mod rubes_world;

use crate::debug::DebugPlugin;
use crate::rubes_world::RubesWorldPlugin;
use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugins(RubesWorldPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
