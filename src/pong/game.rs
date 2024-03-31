use bevy::prelude::*;

use super::{ball::Ball, paddle::Paddle};

#[derive(Event, Default)]
pub struct NewRoundEvent;

#[derive(Event, Default)]
pub struct GoalEvent;

#[derive(Resource)]
pub struct GameStatus {
    pub pause_collisions: bool,
}

fn goal(
    mut goal_event: EventReader<GoalEvent>,
    mut new_round_event: EventWriter<NewRoundEvent>,
    mut game_status: ResMut<GameStatus>,
) {
    if !goal_event.is_empty() {
        goal_event.clear();
        game_status.pause_collisions = true;

        new_round_event.send_default();
    }
}

fn new_round(
    mut new_round_event: EventReader<NewRoundEvent>,
    mut paddles_query: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), (With<Ball>, Without<Paddle>)>,
    mut game_status: ResMut<GameStatus>,
) {
    if !new_round_event.is_empty() {
        new_round_event.clear();

        for mut paddle in paddles_query.iter_mut() {
            paddle.translation.y = 0.;
        }

        let (mut ball_transform, mut ball) = ball_query.single_mut();

        ball_transform.translation = Vec3::ZERO;
        ball.reset();
        game_status.pause_collisions = false;
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewRoundEvent>()
            .add_event::<GoalEvent>()
            .insert_resource(GameStatus {
                pause_collisions: false,
            })
            .add_systems(Update, (new_round, goal));
    }
}
