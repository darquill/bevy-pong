pub mod camera;
pub mod paddle;

use bevy::prelude::*;
use camera::MainCameraPlugin;
use paddle::PaddlePlugin;

pub struct PongGame;

impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainCameraPlugin, PaddlePlugin));
    }
}
