use bevy::prelude::Component;

// The derived default values of numeric fields in Rust are zero
#[derive(Default, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
