mod config;
mod pong;

use bevy::prelude::*;
use config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pong::PongGame;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusty Pong".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            PongGame,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
