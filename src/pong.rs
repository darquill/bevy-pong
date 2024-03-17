pub mod ball;
pub mod camera;
pub mod controls;
pub mod paddle;

use ball::BallPlugin;
use bevy::prelude::*;
use camera::MainCameraPlugin;
use controls::GameControlsPlugin;
use paddle::PaddlePlugin;

pub struct PongGame;

impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainCameraPlugin,
            PaddlePlugin,
            GameControlsPlugin,
            BallPlugin,
        ));
    }
}
