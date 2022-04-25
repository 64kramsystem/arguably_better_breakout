mod camera;

mod in_game {
    mod background;
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

mod main_menu {
    mod background;
    mod menu;
    pub mod module_plugin;
}

use bevy::{
    prelude::{App, IntoSystem},
    DefaultPlugins,
};

use camera::spawn_cameras;
use in_game::module_plugin::InGamePlugin;
use main_menu::module_plugin::MainMenuPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

/// A simple implementation of the classic game "Breakout"
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_cameras.system())
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGamePlugin)
        .run();
}
