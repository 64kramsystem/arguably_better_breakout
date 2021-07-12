use bevy::{
    input::Input,
    math::{const_quat, const_vec2, const_vec3, Vec2},
    prelude::{
        Assets, Bundle, Color, Commands, KeyCode, Query, Res, ResMut, SpriteBundle, Transform, With,
    },
    sprite::{ColorMaterial, Sprite},
};

use crate::{collides::Collides, velocity::Velocity, TIME_STEP};

const PADDLE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const PADDLE_SPEED: f32 = 20000.0;
const PADDLE_SIZE: Vec2 = const_vec2!([120.0, 30.0]);
const PADDLE_BOUND: f32 = 380.0;
const PADDLE_STARTING_TRANSFORM: Transform = Transform {
    translation: const_vec3!([0.0, -215.0, 0.0]),
    // We don't want any rotation
    rotation: const_quat!([0.0, 0.0, 0.0, 0.0]),
    // We want the scale to be 1 in all directions
    scale: const_vec3!([1.0, 1.0, 1.0]),
};

pub struct Paddle {
    pub speed: f32,
}

#[derive(Bundle)]
struct PaddleBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    paddle: Paddle,
    collides: Collides,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                material: materials.add(PADDLE_COLOR.into()),
                transform: PADDLE_STARTING_TRANSFORM,
                sprite: Sprite::new(PADDLE_SIZE),
                ..Default::default()
            },
            paddle: Paddle {
                speed: PADDLE_SPEED,
            },
            collides: Collides,
            velocity: Velocity::default(),
        }
    }
}

pub fn spawn_paddle(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(PaddleBundle::new(materials));
}

/// Reads left and right arrow key inputs to set paddle velocity
pub fn paddle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Velocity)>,
) {
    let (paddle, mut velocity) = query.single_mut().unwrap();

    let mut direction = 0.0;
    // Adds to the direction rather than just setting it to
    // properly handle case where both are pressed at once
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    velocity.x = direction * paddle.speed * TIME_STEP;
}

/// Ensures our paddle never goes out of bounds
pub fn bound_paddle(mut query: Query<(&mut Transform, &mut Velocity), With<Paddle>>) {
    let (mut paddle_transform, mut paddle_velocity) = query.single_mut().unwrap();

    if paddle_transform.translation.x >= PADDLE_BOUND {
        paddle_transform.translation.x = PADDLE_BOUND;
        paddle_velocity.x = 0.0;
    } else if paddle_transform.translation.x <= -PADDLE_BOUND {
        paddle_transform.translation.x = -PADDLE_BOUND;
        paddle_velocity.x = 0.0;
    }
}
