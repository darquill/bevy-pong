mod pong;

use bevy::prelude::*;
use pong::camera::MainCameraPlugin;
use pong::paddle::Paddle;

fn setup(mut commands: Commands) {
    commands.spawn(Paddle);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MainCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
