use bevy::prelude::{Commands, OrthographicCameraBundle, UiCameraBundle};

pub fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
