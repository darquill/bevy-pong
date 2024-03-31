use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::config::{GAME_DIFFICULTY, WALL_HEIGHT, WINDOW_HEIGHT};

use super::{
    ball::Ball,
    collisions::Collider,
    game::GameStatus,
    paddle::{AiController, Paddle},
};

fn move_paddle(
    mut ai_paddles: Query<(&mut Transform, &Paddle, &mut Collider), With<AiController>>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
    time: Res<Time>,
    game_status: Res<GameStatus>,
) {
    if game_status.pause {
        return;
    }

    let ball_transform = ball_query.single();
    let (mut paddle_transform, paddle, mut collider) = ai_paddles.single_mut();

    paddle_transform.translation = paddle_transform
        .translation
        .lerp(
            Vec3::new(
                paddle_transform.translation.x,
                ball_transform.translation.y,
                0.0,
            ),
            GAME_DIFFICULTY * time.delta_seconds(),
        )
        .clamp(
            Vec3 {
                x: paddle_transform.translation.x,
                y: -WINDOW_HEIGHT / 2. + paddle.size.y / 2. + WALL_HEIGHT,
                z: 0.,
            },
            Vec3 {
                x: paddle_transform.translation.x,
                y: WINDOW_HEIGHT / 2. - paddle.size.y / 2. - WALL_HEIGHT,
                z: 0.,
            },
        );

    collider.collision_rect =
        Aabb2d::new(paddle_transform.translation.truncate(), paddle.size / 2.);
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_paddle);
    }
}
