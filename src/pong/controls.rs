use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::config::{WALL_HEIGHT, WINDOW_HEIGHT};

use super::{
    collisions::Collider,
    game::{GameStatus, NewRoundEvent},
    paddle::{Paddle, PlayerController},
};
pub struct GameControlsPlugin;

fn move_paddle(
    mut paddles: Query<
        (&mut Transform, &mut Paddle, &mut Collider),
        (With<Paddle>, With<PlayerController>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_status: Res<GameStatus>,
) {
    if game_status.pause {
        return;
    }

    let (mut paddle_transform, paddle, mut collider) = paddles.single_mut();

    let mut direction = 0.;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction = 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction = -1.;
    }

    paddle_transform.translation = paddle_transform
        .translation
        .lerp(
            Vec3::new(
                paddle_transform.translation.x,
                paddle_transform.translation.y + paddle.speed * direction * time.delta_seconds(),
                0.0,
            ),
            paddle.speed,
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

fn new_round(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut new_round_event: EventWriter<NewRoundEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        new_round_event.send_default();
    }
}

impl Plugin for GameControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_paddle, new_round));
    }
}
