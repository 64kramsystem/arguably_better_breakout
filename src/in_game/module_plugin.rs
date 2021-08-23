use bevy::{core::FixedTimestep, input::system::exit_on_esc_system, prelude::*};

use crate::AppState;

use super::background::set_background;
use super::ball::{ball_collision, spawn_ball};
use super::brick::spawn_bricks;
use super::kinematics::kinematics;
use super::paddle::{bound_paddle, paddle_input, spawn_paddle};
use super::score::{init_score, spawn_scoreboard, update_scoreboard};
use super::wall::spawn_walls;

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                // This adds the Score resource with its default value of 0
                // These systems run only once, before all other systems
                .with_system(init_score.system())
                .with_system(set_background.system())
                .with_system(spawn_paddle.system())
                .with_system(spawn_ball.system())
                .with_system(spawn_walls.system())
                .with_system(spawn_bricks.system())
                .with_system(spawn_scoreboard.system()),
        );
    }
}

struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(1. / 50.))
                .with_system(kinematics.system().label("kinematics"))
                // We need to check for collisions before handling movement
                // to reduce the risk of the ball passing through objects
                .with_system(ball_collision.system().before("kinematics"))
                .with_system(
                    bound_paddle
                        .system()
                        .label("bound_paddle")
                        // This system must run after kinematics, or the velocity will be set to 0
                        // before the paddle moves, causing it to be stuck to the wall
                        .after("kinematics"),
                ),
        );
    }
}

struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                // We need to handle input before we move our paddle,
                // to ensure that we're responding to the most recent frame's events,
                // avoiding input lag
                // See https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs
                // for more information on system ordering
                .with_system(
                    paddle_input
                        .system()
                        .before("bound_paddle")
                        .before("kinematics"),
                )
                // Exits the game when `KeyCode::Esc` is pressed
                // This is a simple built-in system
                .with_system(exit_on_esc_system.system()),
        );
    }
}

struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(update_scoreboard.system()),
        );
    }
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SetupPlugin)
            .add_plugin(KinematicsPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(ScorePlugin);
    }
}
