use bevy::prelude::*;

use super::{
    ball::Ball,
    paddle::{AiController, Paddle},
};

fn move_paddle(
    mut ai_paddles: Query<(&mut Transform, &Paddle), With<AiController>>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
) {
    let ball = ball_query.single();
    let direction = ball.translation.y;
    let (mut paddle_transform, _paddle) = ai_paddles.single_mut();

    paddle_transform.translation.y = direction;
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_paddle);
    }
}
