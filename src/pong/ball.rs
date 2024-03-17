use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct BallPlugin;

const BALL_SIZE: Vec2 = Vec2::new(1.0, 1.0);

#[derive(Component)]
pub struct Ball;

fn add_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("[PLUGIN:PaddlePlugin] add_ball");

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(10.0))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: BALL_SIZE.extend(1.0),
                ..default()
            },
            ..default()
        },
        Ball,
    ));
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ball);
    }
}
