use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    config::{PADDLE_SIZE, PADDLE_SPEED, WINDOW_WIDTH},
    pong::collisions::Collider,
};

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct AiController;

#[derive(Component, Clone, Debug)]
pub struct Paddle {
    pub speed: f32,
    pub size: Vec2,
}

#[derive(Component)]
pub struct DebugPaddle;

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            speed: PADDLE_SPEED,
            size: PADDLE_SIZE,
        }
    }
}

fn add_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let center: Vec3 = Vec3::new(-WINDOW_WIDTH / 2. + PADDLE_SIZE.x, 0.0, 0.0);
    // P1 Paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(PADDLE_SIZE))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
        Paddle::new(),
        PlayerController,
        Collider::new(center.truncate(), PADDLE_SIZE),
    ));

    let center: Vec3 = Vec3::new(WINDOW_WIDTH / 2. - PADDLE_SIZE.x, 0.0, 0.0);
    // P2 Paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(PADDLE_SIZE))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
        Paddle::new(),
        AiController,
        Collider::new(center.truncate(), PADDLE_SIZE),
    ));
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_paddles);
    }
}
