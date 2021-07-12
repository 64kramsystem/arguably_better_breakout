use bevy::{
    input::Input,
    prelude::{Assets, Commands, KeyCode, Query, Res, ResMut, SpriteBundle, Transform, With},
    sprite::{ColorMaterial, Sprite},
};

use crate::{collides::Collides, config::*, velocity::Velocity};

pub struct Paddle {
    pub speed: f32,
}

pub fn spawn_paddle(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(PADDLE_COLOR.into()),
            transform: PADDLE_STARTING_TRANSFORM,
            sprite: Sprite::new(PADDLE_SIZE),
            ..Default::default()
        })
        .insert(Paddle {
            speed: PADDLE_SPEED,
        })
        .insert(Collides)
        .insert(Velocity::default());
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
