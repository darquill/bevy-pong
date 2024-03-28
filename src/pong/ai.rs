use bevy::{math::bounding::Aabb2d, prelude::*};

use super::{
    ball::Ball,
    collisions::Collider,
    paddle::{AiController, Paddle},
};

fn move_paddle(
    mut ai_paddles: Query<(&mut Transform, &Paddle, &mut Collider), With<AiController>>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
) {
    let ball = ball_query.single();
    let direction = ball.translation.y;
    let (mut paddle_transform, paddle, mut collider) = ai_paddles.single_mut();

    paddle_transform.translation.y = direction;

    collider.collision_rect =
        Aabb2d::new(paddle_transform.translation.truncate(), paddle.size / 2.);
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_paddle);
    }
}
