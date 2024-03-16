mod pong;

use bevy::prelude::*;
use pong::PongGame;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PongGame))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
