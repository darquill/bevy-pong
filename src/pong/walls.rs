use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::config::{WALL_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::collisions::Collider;

#[derive(Component)]
pub struct Wall;

fn create_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2 {
        x: WINDOW_WIDTH,
        y: WALL_HEIGHT * 4.,
    };

    let display_size = size + Vec2 { x: 0., y: 0. };

    let center = Vec3 {
        x: 0.,
        y: WINDOW_HEIGHT / 2. + 50.0,
        z: 0.,
    };

    commands.spawn((
        Wall,
        Collider::new(
            center.truncate(),
            Vec2 {
                x: WINDOW_WIDTH,
                y: WALL_HEIGHT * 4.,
            },
        ),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(display_size))),
            material: materials.add(Color::GRAY),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
    ));

    let center = Vec3 {
        x: 0.,
        y: -WINDOW_HEIGHT / 2. - 50.,
        z: 0.,
    };

    commands.spawn((
        Wall,
        Collider::new(center.truncate(), size),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(display_size))),
            material: materials.add(Color::GRAY),
            transform: Transform {
                translation: center,
                ..default()
            },
            ..default()
        },
    ));
}
pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_walls);
    }
}
