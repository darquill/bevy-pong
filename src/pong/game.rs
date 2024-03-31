use bevy::prelude::*;

use super::{ball::Ball, paddle::Paddle};

#[derive(Event, Default)]
pub struct NewRoundEvent;

#[derive(Event, Default)]
pub struct GoalEvent;

#[derive(Resource)]
pub struct GameStatus {
    pub pause: bool,
}

#[derive(Resource)]
struct NewRoundTimer(Timer);

fn goal(mut goal_event: EventReader<GoalEvent>, mut new_round_event: EventWriter<NewRoundEvent>) {
    if !goal_event.is_empty() {
        goal_event.clear();
        new_round_event.send_default();
    }
}

fn new_round(
    mut new_round_event: EventReader<NewRoundEvent>,
    mut paddles_query: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), (With<Ball>, Without<Paddle>)>,
    mut game_status: ResMut<GameStatus>,
    mut new_round_timer: ResMut<NewRoundTimer>,
) {
    if !new_round_event.is_empty() {
        new_round_event.clear();

        game_status.pause = true;
        new_round_timer.0.reset();

        for mut paddle in paddles_query.iter_mut() {
            paddle.translation.y = 0.;
        }

        let (mut ball_transform, mut ball) = ball_query.single_mut();

        ball_transform.translation = Vec3::ZERO;
        ball.reset();
    }
}

fn countdown_new_round(
    time: Res<Time>,
    mut new_round_timer: ResMut<NewRoundTimer>,
    mut game_status: ResMut<GameStatus>,
) {
    new_round_timer.0.tick(time.delta());

    if new_round_timer.0.just_finished() {
        game_status.pause = false;
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewRoundEvent>()
            .add_event::<GoalEvent>()
            .insert_resource(GameStatus { pause: true })
            .insert_resource(NewRoundTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_systems(Update, (countdown_new_round, new_round, goal).chain());
    }
}
