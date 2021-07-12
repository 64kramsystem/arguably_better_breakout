use bevy::{
    math::Vec2,
    prelude::{Assets, Bundle, Commands, Handle, ResMut, SpriteBundle, Transform},
    sprite::{ColorMaterial, Sprite},
};

use crate::{collides::Collides, config::*};

pub struct Brick;

#[derive(Bundle)]
struct BrickBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    brick: Brick,
    collides: Collides,
}

impl BrickBundle {
    fn new(x: f32, y: f32, material_handle: Handle<ColorMaterial>) -> Self {
        BrickBundle {
            sprite_bundle: SpriteBundle {
                material: material_handle,
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite::new(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                ..Default::default()
            },
            brick: Brick,
            collides: Collides,
        }
    }
}

pub fn spawn_bricks(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let brick_material = materials.add(BRICK_COLOR.into());

    // Compute the total width that all of the bricks take
    const TOTAL_WIDTH: f32 = BRICK_COLUMNS as f32 * (BRICK_WIDTH + BRICK_SPACING) - BRICK_SPACING;
    // Center the bricks
    const OFFSET_X: f32 = -(TOTAL_WIDTH - BRICK_WIDTH) / 2.0;
    // Move the bricks up slightly
    const OFFSET_Y: f32 = 100.0;

    // Add the bricks
    let brick_iterator = (0..BRICK_ROWS)
        .flat_map(|row| (0..BRICK_COLUMNS).map(move |col| (row, col)))
        .map(move |(row, column)| {
            BrickBundle::new(
                column as f32 * (BRICK_WIDTH + BRICK_SPACING) + OFFSET_X,
                row as f32 * (BRICK_HEIGHT + BRICK_SPACING) + OFFSET_Y,
                brick_material.clone(),
            )
        });
    // spawn_batch is slightly more efficient than repeatedly calling .spawn_bundle due to memory pre-allocation
    // This approach is overkill for the small number of entities here, but serves to demonstrate how the function is used
    commands.spawn_batch(brick_iterator);

    /* Equivalently, you could spawn one brick at a time using for loops instead, at a small cost to performance
    for row in 0..BRICK_ROWS {
        for column in 0..BRICK_COLUMNS {
            commands.spawn_bundle(BrickBundle::new(
                column as f32 * (BRICK_WIDTH + BRICK_SPACING) + OFFSET_X,
                row as f32 * (BRICK_HEIGHT + BRICK_SPACING) + OFFSET_Y,
                &brick_material,
            ));
        }
    }
    */
}
