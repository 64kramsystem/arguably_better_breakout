use bevy::prelude::{ClearColor, Color, Commands};

const BACKGROUND_COLOR: ClearColor = ClearColor(Color::rgb(0.9, 0.9, 0.9));

pub fn set_background(mut commands: Commands) {
    commands.insert_resource(BACKGROUND_COLOR);
}
