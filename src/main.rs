mod in_game {
    mod ball;
    mod brick;
    mod camera;
    mod collides;
    mod kinematics;
    pub mod module_plugin;
    mod paddle;
    mod score;
    mod velocity;
    mod wall;
}

use bevy::{prelude::App, DefaultPlugins};

use in_game::module_plugin::InGamePlugin;

/// A simple implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InGamePlugin)
        .run();
}
