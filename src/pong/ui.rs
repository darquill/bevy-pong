use bevy::prelude::*;

use super::game::Score;

#[derive(Component)]
pub struct P1Score;

#[derive(Component)]
pub struct CPUScore;

#[derive(Event)]
pub struct UpdateScoreEvent(pub Score);

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "P1: 0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(5.),
            top: Val::Px(5.),
            ..default()
        }),
        P1Score,
    ));

    commands.spawn((
        TextBundle::from_section(
            "CPU: 0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(5.),
            top: Val::Px(5.),
            ..default()
        }),
        CPUScore,
    ));
}

fn update_scores(
    mut p1_score_ui_query: Query<&mut Text, (With<P1Score>, Without<CPUScore>)>,
    mut cpu_score_ui_query: Query<&mut Text, (With<CPUScore>, Without<P1Score>)>,
    mut score_event: EventReader<UpdateScoreEvent>,
) {
    if score_event.is_empty() {
        return;
    }

    for ev in score_event.read() {
        let mut p1_score_ui_text = p1_score_ui_query.single_mut();
        let mut cpu_score_ui_text = cpu_score_ui_query.single_mut();

        p1_score_ui_text.sections[0].value = format!("P1: {}", ev.0.p1_score);

        cpu_score_ui_text.sections[0].value = format!("CPU: {}", ev.0.cpu_score);
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<UpdateScoreEvent>()
            .add_systems(Update, update_scores);
    }
}
