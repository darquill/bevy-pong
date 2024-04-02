pub mod ai;
pub mod ball;
pub mod camera;
pub mod collisions;
pub mod controls;
pub mod game;
pub mod goals;
pub mod paddle;
pub mod ui;
pub mod walls;

use ai::AiPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use camera::MainCameraPlugin;
use collisions::CollisionsPlugin;
use controls::GameControlsPlugin;
use game::GamePlugin;
use goals::GoalsPlugin;
use paddle::PaddlePlugin;
use ui::UiPlugin;
use walls::WallsPlugin;

pub struct PongGame;

impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainCameraPlugin,
            PaddlePlugin,
            GameControlsPlugin,
            BallPlugin,
            CollisionsPlugin,
            AiPlugin,
            WallsPlugin,
            GamePlugin,
            GoalsPlugin,
            UiPlugin,
        ));
    }
}
