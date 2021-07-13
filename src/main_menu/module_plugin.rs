use bevy::prelude::*;

use crate::AppState;

use super::{
    background::set_background,
    menu::{spawn_menu, start_game_on_enter},
};

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(set_background.system())
                .with_system(spawn_menu.system()),
        );
    }
}

struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(start_game_on_enter.system()),
        );
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SetupPlugin).add_plugin(InputPlugin);
    }
}
