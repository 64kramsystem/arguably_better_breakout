mod ball;
mod brick;
mod camera;
mod collides;
mod kinematics;
mod paddle;
mod score;
mod velocity;
mod wall;

use ball::{ball_collision, spawn_ball};
use bevy::{input::system::exit_on_esc_system, prelude::*, render::pass::ClearColor};

use brick::spawn_bricks;
use camera::spawn_cameras;
use kinematics::kinematics;
use paddle::{bound_paddle, paddle_input, spawn_paddle};
use score::{spawn_scoreboard, update_scoreboard, Score};
use velocity::Velocity;
use wall::spawn_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

/// A simple implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // This adds the Score resource with its default value of 0
        .init_resource::<Score>()
        // These systems run only once, before all other systems
        .add_startup_system(spawn_cameras.system())
        .add_startup_system(spawn_paddle.system())
        .add_startup_system(spawn_ball.system())
        .add_startup_system(spawn_walls.system())
        .add_startup_system(spawn_bricks.system())
        .add_startup_system(spawn_scoreboard.system())
        .add_system(kinematics.system().label("kinematics"))
        // We need to check for collisions before handling movement
        // to reduce the risk of the ball passing through objects
        .add_system(ball_collision.system().before("kinematics"))
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
        .add_system(
            bound_paddle
                .system()
                .label("bound_paddle")
                // This system must run after kinematics, or the velocity will be set to 0
                // before the paddle moves, causing it to be stuck to the wall
                .after("kinematics"),
        )
        .add_system(update_scoreboard.system())
        // Exits the game when `KeyCode::Esc` is pressed
        // This is a simple built-in system
        .add_system(exit_on_esc_system.system())
        .run();
}
