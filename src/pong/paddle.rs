use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PADDLE_SIZE: Vec2 = Vec2::new(1.0, 2.0);

#[derive(Component)]
pub struct Controller {
    player: bool,
}

#[derive(Bundle)]
pub struct Paddle {
    controller: Controller,
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
        Paddle {
            controller: Controller { player: true },
        },
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
        Paddle {
            controller: Controller { player: true },
        },
    ));
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_paddles);
    }
}
