mod camera;

mod in_game {
    mod ball;
    mod brick;
    mod collides;
    mod kinematics;
    pub mod module_plugin;
    mod paddle;
    mod score;
    mod velocity;
    mod wall;
}

use bevy::{
    prelude::{App, IntoSystem},
    DefaultPlugins,
};

use camera::spawn_cameras;
use in_game::module_plugin::InGamePlugin;

/// A simple implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_cameras.system())
        .add_plugin(InGamePlugin)
        .run();
}
