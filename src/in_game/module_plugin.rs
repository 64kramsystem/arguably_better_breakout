use bevy::{input::system::exit_on_esc_system, prelude::*};

use super::background::set_background;
use super::ball::{ball_collision, spawn_ball};
use super::brick::spawn_bricks;
use super::kinematics::kinematics;
use super::paddle::{bound_paddle, paddle_input, spawn_paddle};
use super::score::{spawn_scoreboard, update_scoreboard, Score};
use super::wall::spawn_walls;

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // This adds the Score resource with its default value of 0
            .init_resource::<Score>()
            // These systems run only once, before all other systems
            .add_startup_system(set_background.system())
            .add_startup_system(spawn_paddle.system())
            .add_startup_system(spawn_ball.system())
            .add_startup_system(spawn_walls.system())
            .add_startup_system(spawn_bricks.system())
            .add_startup_system(spawn_scoreboard.system());
    }
}

struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(kinematics.system().label("kinematics"))
            // We need to check for collisions before handling movement
            // to reduce the risk of the ball passing through objects
            .add_system(ball_collision.system().before("kinematics"))
            .add_system(
                bound_paddle
                    .system()
                    .label("bound_paddle")
                    // This system must run after kinematics, or the velocity will be set to 0
                    // before the paddle moves, causing it to be stuck to the wall
                    .after("kinematics"),
            );
    }
}

struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // We need to handle input before we move our paddle,
            // to ensure that we're responding to the most recent frame's events,
            // avoiding input lag
            // See https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs
            // for more information on system ordering
            .add_system(
                paddle_input
                    .system()
                    .before("bound_paddle")
                    .before("kinematics"),
            )
            // Exits the game when `KeyCode::Esc` is pressed
            // This is a simple built-in system
            .add_system(exit_on_esc_system.system());
    }
}

struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_scoreboard.system());
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
