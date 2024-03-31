use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::config::WINDOW_HEIGHT;

use super::{
    ball::Ball,
    collisions::Collider,
    paddle::{AiController, Paddle},
};

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
    mut ai_paddles: Query<(&mut Transform, &Paddle, &mut Collider), With<AiController>>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
    time: Res<Time>,
) {
    let ball_transform = ball_query.single();
    let (mut paddle_transform, paddle, mut collider) = ai_paddles.single_mut();

    let direction = match ball_transform.translation.y > paddle_transform.translation.y {
        true => 1.,
        false => -1.,
    };

    let offset = direction * paddle.speed * time.delta_seconds();

    let oob = check_oob(paddle_transform.translation.y, paddle.size.y, offset);

    if oob.is_some() {
        return;
    }

    paddle_transform.translation = paddle_transform.translation.lerp(
        Vec3::new(
            paddle_transform.translation.x,
            ball_transform.translation.y,
            0.0,
        ),
        0.1,
    );
    // paddle_transform.translation.y += offset;

    collider.collision_rect =
        Aabb2d::new(paddle_transform.translation.truncate(), paddle.size / 2.);
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_paddle);
    }
}
