// Constants that can be used to fine-tune the behavior of our game

use bevy::math::{const_quat, const_vec2, const_vec3, Vec2};
use bevy::render::color::Color;
use bevy::transform::components::Transform;
use bevy::ui::Val;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const PADDLE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const PADDLE_SPEED: f32 = 20000.0;
pub const PADDLE_SIZE: Vec2 = const_vec2!([120.0, 30.0]);
pub const PADDLE_BOUND: f32 = 380.0;
pub const PADDLE_STARTING_TRANSFORM: Transform = Transform {
    translation: const_vec3!([0.0, -215.0, 0.0]),
    // We don't want any rotation
    rotation: const_quat!([0.0, 0.0, 0.0, 0.0]),
    // We want the scale to be 1 in all directions
    scale: const_vec3!([1.0, 1.0, 1.0]),
};

pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// Our ball is actually a square. Shhh...
pub const BALL_SIZE: Vec2 = const_vec2!([30.0, 30.0]);
pub const BALL_STARTING_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);
pub const BALL_STARTING_SPEED: f32 = 400.0;
// We set the z-value to one to ensure it appears on top of our other objects in case of overlap
pub const BALL_STARTING_TRANSFORM: Transform = Transform {
    translation: const_vec3!([0.0, -50.0, 1.0]),
    rotation: const_quat!([0.0, 0.0, 0.0, 0.0]),
    scale: const_vec3!([1.0, 1.0, 1.0]),
};

pub const ARENA_BOUNDS: Vec2 = const_vec2!([900.0, 600.0]);
pub const WALL_THICKNESS: f32 = 10.0;
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const BRICK_WIDTH: f32 = 150.0;
pub const BRICK_HEIGHT: f32 = 30.0;
pub const BRICK_ROWS: i8 = 4;
pub const BRICK_COLUMNS: i8 = 5;
pub const BRICK_SPACING: f32 = 20.0;

pub const SCOREBOARD_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const SCORE_FONT_SIZE: f32 = 40.0;
pub const SCORE_PADDING: Val = Val::Px(5.0);
pub const SCOREBOARD_FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";
pub const SCORE_FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";
