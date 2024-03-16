use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PADDLE_SIZE: Vec2 = Vec2::new(1.0, 2.0);
const PADDLE_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct AiController;

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            speed: PADDLE_SPEED,
        }
    }
}

fn add_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("[PLUGIN:PaddlePlugin] add_paddles");

    // P1 Paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 20.0))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            ..default()
        },
        Paddle::new(),
        PlayerController,
    ));

    // P2 Paddle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 20.0))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: Vec3::new(200.0, 0.0, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            ..default()
        },
        Paddle::new(),
        AiController,
    ));
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_paddles);
    }
}
