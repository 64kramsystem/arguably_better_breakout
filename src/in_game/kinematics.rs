use bevy::{
    core::Time,
    prelude::{Local, Query, Res, Transform},
};

use super::velocity::Velocity;

/// Moves everything with both a Transform and a Velocity accordingly
pub fn kinematics(
    mut last_time: Local<f64>,
    mut post_first_invocation: Local<bool>,
    time: Res<Time>,
    query: Query<(&mut Transform, &Velocity)>,
) {
    let mut last_time_diff = time.seconds_since_startup() - *last_time;

    // When switching state, the first invocation will result in a large time diff, which causes issues.
    //
    // As of Jul/2021, this is a known issue (https://discord.com/channels/691052431525675048/742884593551802431/864911869461528657).
    //
    // > last_time starts at 0
    // > which means if you use states, it wants to compensate for all of the time spent outside it
    //
    if !*post_first_invocation {
        last_time_diff = last_time_diff.clamp(0., 0.333);
        *post_first_invocation = true;
    }

    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x * last_time_diff as f32;
        transform.translation.y += velocity.y * last_time_diff as f32;
    });

    *last_time = time.seconds_since_startup();
}
