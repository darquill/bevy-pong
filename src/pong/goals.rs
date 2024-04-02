use super::collisions::Collider;
use crate::config::{GOAL_WIDTH, WALL_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct P1Goal;

#[derive(Component)]
pub struct CPUGoal;

fn create_goals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2 {
        x: GOAL_WIDTH * 4.,
        y: WINDOW_HEIGHT - WALL_HEIGHT * 2.,
    };

    let display_size = size + Vec2 { x: 0., y: 0. };

    let center = Vec3 {
        x: -WINDOW_WIDTH / 2. - GOAL_WIDTH * 2. + 2.,
        y: 0.,
        z: 0.,
    };

    commands.spawn((
        P1Goal,
        Collider::new(center.truncate(), size),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(display_size))),
            material: materials.add(Color::ORANGE_RED),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
    ));

    let center = Vec3 {
        x: WINDOW_WIDTH / 2. + GOAL_WIDTH * 2. - 2.,
        y: 0.,
        z: 0.,
    };

    commands.spawn((
        CPUGoal,
        Collider::new(center.truncate(), size),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(display_size))),
            material: materials.add(Color::SEA_GREEN),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
    ));
}

pub struct GoalsPlugin;

impl Plugin for GoalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_goals);
    }
}
