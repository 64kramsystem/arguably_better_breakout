use bevy::{
    math::{const_vec2, Vec2},
    prelude::{Assets, Bundle, Color, Commands, Handle, ResMut, SpriteBundle, Transform},
    sprite::{ColorMaterial, Sprite},
};

use super::collides::Collides;

/// Defines which side of the arena a wall is part of
pub const ARENA_BOUNDS: Vec2 = const_vec2!([900.0, 600.0]);
pub const WALL_THICKNESS: f32 = 10.0;
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    pub fn wall_coord(&self, bounds: Vec2) -> Transform {
        let (x, y) = match self {
            Side::Top => (0.0, bounds.y / 2.0),
            Side::Bottom => (0.0, -bounds.y / 2.0),
            Side::Left => (-bounds.x / 2.0, 0.0),
            Side::Right => (bounds.x / 2.0, 0.0),
        };
        // We need to convert these coordinates into a 3D transform to add to our SpriteBundle
        Transform::from_xyz(x, y, 0.0)
    }

    pub fn wall_size(&self, bounds: Vec2, thickness: f32) -> Vec2 {
        match self {
            Side::Top => Vec2::new(bounds.x + thickness, thickness),
            Side::Bottom => Vec2::new(bounds.x + thickness, thickness),
            Side::Left => Vec2::new(thickness, bounds.y + thickness),
            Side::Right => Vec2::new(thickness, bounds.y + thickness),
        }
    }
}

// By creating our own bundles, we can avoid duplicating code
#[derive(Bundle)]
struct WallBundle {
    // Use #[bundle] like this to nest bundles correctly
    #[bundle]
    sprite_bundle: SpriteBundle,
    collides: Collides,
}

impl WallBundle {
    fn new(side: Side, material_handle: Handle<ColorMaterial>) -> Self {
        WallBundle {
            sprite_bundle: SpriteBundle {
                material: material_handle,
                transform: side.wall_coord(ARENA_BOUNDS),
                sprite: Sprite::new(side.wall_size(ARENA_BOUNDS, WALL_THICKNESS)),
                ..Default::default()
            },
            collides: Collides,
        }
    }
}

pub fn spawn_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_handle = materials.add(WALL_COLOR.into());

    // Each material handle must be uniquely owned as handles are ref-counted
    commands.spawn_bundle(WallBundle::new(Side::Top, material_handle.clone()));
    commands.spawn_bundle(WallBundle::new(Side::Bottom, material_handle.clone()));
    commands.spawn_bundle(WallBundle::new(Side::Left, material_handle.clone()));
    commands.spawn_bundle(WallBundle::new(Side::Right, material_handle));
}
