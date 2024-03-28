use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::{thread_rng, Rng};

use super::{
    ball::Ball,
    paddle::{Paddle, PlayerController},
};
pub struct GameControlsPlugin;

fn move_paddle(
    mut paddles: Query<(&mut Transform, &mut Paddle), (With<Paddle>, With<PlayerController>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut paddle_transform, mut paddle) = paddles.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        paddle_transform.translation.y += paddle.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        paddle_transform.translation.y -= paddle.speed * time.delta_seconds();
    }

    paddle.collision_rect = Aabb2d::new(paddle_transform.translation.truncate(), paddle.size / 2.);

    // TODO: Add clamp to avoid paddle moving beyond screen borders.
}

fn reset_ball(
    mut ball: Query<(&mut Transform, &mut Ball), With<Ball>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut rng = thread_rng();

    let (mut ball_transform, mut ball) = ball.single_mut();
    if keyboard_input.pressed(KeyCode::KeyX) {
        ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
        ball.velocity = Vec2::new(-1.0, rng.gen_range(-0.5..0.5));
    }
}

impl Plugin for GameControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (move_paddle, reset_ball));
    }
}
