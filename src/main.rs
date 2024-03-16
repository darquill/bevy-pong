mod camera;
mod paddle;

use bevy::prelude::*;
use camera::MainCameraPlugin;
use paddle::Paddle;

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
