use bevy::prelude::{Query, Transform};

use crate::{config::*, velocity::Velocity};

/// Moves everything with both a Transform and a Velocity accordingly
pub fn kinematics(query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    });
}
