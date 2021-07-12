use bevy::{
    core::Time,
    prelude::{Local, Query, Res, Transform},
};

use crate::velocity::Velocity;

/// Moves everything with both a Transform and a Velocity accordingly
pub fn kinematics(
    mut last_time: Local<f64>,
    time: Res<Time>,
    query: Query<(&mut Transform, &Velocity)>,
) {
    let last_time_diff = time.seconds_since_startup() - *last_time;

    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x * last_time_diff as f32;
        transform.translation.y += velocity.y * last_time_diff as f32;
    });

    *last_time = time.seconds_since_startup();
}
