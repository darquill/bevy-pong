use bevy::prelude::*;

use super::{ball::Ball, paddle::Paddle};

#[derive(Event, Default)]
pub struct NewRoundEvent;

fn new_round(
    mut new_round_event: EventReader<NewRoundEvent>,
    mut paddles_query: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), (With<Ball>, Without<Paddle>)>,
) {
    if !new_round_event.is_empty() {
        new_round_event.clear();

        for mut paddle in paddles_query.iter_mut() {
            paddle.translation.y = 0.;
        }

        let (mut ball_transform, mut ball) = ball_query.single_mut();

        ball_transform.translation = Vec3::ZERO;
        ball.reset();
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewRoundEvent>()
            .add_systems(Update, new_round);
    }
}
