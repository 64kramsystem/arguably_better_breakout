use bevy::{
    math::{const_quat, const_vec2, const_vec3, Vec2},
    prelude::{
        Assets, Bundle, Color, Commands, Entity, Query, ResMut, SpriteBundle, Transform, With,
        Without,
    },
    sprite::{
        collide_aabb::{collide, Collision},
        ColorMaterial, Sprite,
    },
};

use crate::{brick::Brick, collides::Collides, score::Score, Velocity};

const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// Our ball is actually a square. Shhh...
const BALL_SIZE: Vec2 = const_vec2!([30.0, 30.0]);
const BALL_STARTING_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);
const BALL_STARTING_SPEED: f32 = 400.0;
// We set the z-value to one to ensure it appears on top of our other objects in case of overlap
const BALL_STARTING_TRANSFORM: Transform = Transform {
    translation: const_vec3!([0.0, -50.0, 1.0]),
    rotation: const_quat!([0.0, 0.0, 0.0, 0.0]),
    scale: const_vec3!([1.0, 1.0, 1.0]),
};

pub struct Ball;

#[derive(Bundle)]
struct BallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    ball: Ball,
    collides: Collides,
    starting_velocity: Velocity,
}

impl BallBundle {
    fn new(ball_starting_velocity: Velocity, mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                material: materials.add(BALL_COLOR.into()),
                transform: BALL_STARTING_TRANSFORM,
                sprite: Sprite::new(BALL_SIZE),
                ..Default::default()
            },
            ball: Ball,
            collides: Collides,
            starting_velocity: ball_starting_velocity,
        }
    }
}

pub fn spawn_ball(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    // .normalize is not a const fn, so we have to perform this operation at runtime
    // FIXME: Blocked on https://github.com/bitshifter/glam-rs/issues/76
    let normalized_direction = BALL_STARTING_DIRECTION.normalize();
    let ball_starting_velocity: Velocity = Velocity {
        x: normalized_direction.x * BALL_STARTING_SPEED,
        y: normalized_direction.y * BALL_STARTING_SPEED,
    };

    commands.spawn_bundle(BallBundle::new(ball_starting_velocity, materials));
}

/// Detects and handles ball collisions
pub fn ball_collision(
    mut ball_query: Query<(&Transform, &mut Velocity, &Sprite), With<Ball>>,
    // Option<&C> returns Some(c: C) if the component exists on the entity, and None if it does not
    collider_query: Query<
        (Entity, &Transform, &Sprite, Option<&Brick>),
        (With<Collides>, Without<Ball>),
    >,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    let (ball_transform, mut ball_velocity, ball_sprite) = ball_query.single_mut().unwrap();
    let ball_size = ball_sprite.size;

    collider_query.for_each(
        |(collider_entity, collider_transform, collider_sprite, maybe_brick)| {
            // Check for collisions
            let collider_size = collider_sprite.size;
            let potential_collision = collide(
                ball_transform.translation,
                ball_size,
                collider_transform.translation,
                collider_size,
            );

            // Handle collisions
            if let Some(collision) = potential_collision {
                // Reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // Only reflect if the ball's velocity is going
                // in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                }

                // Reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // Reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }

                // Perform special brick collision behavior
                if maybe_brick.is_some() {
                    // Despawn bricks that are hit
                    commands.entity(collider_entity).despawn();

                    // Increase the score by 1 for each brick hit
                    score.0 += 1;
                }
            }
        },
    );
}
