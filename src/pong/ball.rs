use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;

pub struct BallPlugin;

const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub size: f32,
}

fn add_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("[PLUGIN:PaddlePlugin] add_ball");
    let mut rng = thread_rng();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_SIZE))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Ball {
            velocity: Vec2::new(-1.0, rng.gen_range(-0.5..0.5)),
            size: BALL_SIZE,
        },
    ));
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Ball), With<Ball>>, time: Res<Time>) {
    let (mut ball_transform, ball) = ball_query.single_mut();

    ball_transform.translation.x += BALL_SPEED * ball.velocity.x * time.delta_seconds();
    ball_transform.translation.y += BALL_SPEED * ball.velocity.y * time.delta_seconds();
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ball)
            .add_systems(Update, move_ball);
    }
}
