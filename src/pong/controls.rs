use bevy::prelude::*;

use super::paddle::{Paddle, PlayerController};
pub struct GameControlsPlugin;

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle), (With<Paddle>, With<PlayerController>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut paddle_transform, paddle) = paddles.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        paddle_transform.translation.y += paddle.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        paddle_transform.translation.y -= paddle.speed * time.delta_seconds();
    }
}

impl Plugin for GameControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_paddle);
    }
}
