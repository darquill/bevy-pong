use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::{thread_rng, Rng};

use crate::config::WINDOW_HEIGHT;

use super::{
    ball::Ball,
    paddle::{Paddle, PlayerController},
};
pub struct GameControlsPlugin;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum OOB {
    Bottom,
    Top,
}

fn check_oob(y: f32, size: f32, offset: f32) -> Option<OOB> {
    if y + (size / 2.) + offset > WINDOW_HEIGHT / 2. - 1. {
        return Some(OOB::Top);
    }

    if y - (size / 2.) + offset < -WINDOW_HEIGHT / 2. + 1. {
        return Some(OOB::Bottom);
    }

    None
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &mut Paddle), (With<Paddle>, With<PlayerController>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut paddle_transform, mut paddle) = paddles.single_mut();

    let offset = paddle.speed * time.delta_seconds();
    let mut direction = 1.;

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction = -1.;
    }

    let oob = check_oob(
        paddle_transform.translation.y,
        paddle.size.y,
        offset * direction,
    );

    if keyboard_input.pressed(KeyCode::KeyW) && oob != Some(OOB::Top) {
        paddle_transform.translation.y += offset;
    }

    if keyboard_input.pressed(KeyCode::KeyS) && oob != Some(OOB::Bottom) {
        paddle_transform.translation.y -= offset;
    }

    paddle.collision_rect = Aabb2d::new(paddle_transform.translation.truncate(), paddle.size / 2.);
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
