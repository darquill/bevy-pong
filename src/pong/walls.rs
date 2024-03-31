use bevy::prelude::*;

use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::collisions::Collider;

#[derive(Component)]
pub struct Wall;

fn create_walls(mut commands: Commands) {
    commands.spawn((
        Wall,
        Collider::new(
            Vec2 {
                x: 0.,
                y: WINDOW_HEIGHT / 2. + 100.,
            },
            Vec2 {
                x: WINDOW_WIDTH,
                y: 200.,
            },
        ),
    ));

    commands.spawn((
        Wall,
        Collider::new(
            Vec2 {
                x: 0.,
                y: -WINDOW_HEIGHT / 2. - 100.,
            },
            Vec2 {
                x: WINDOW_WIDTH,
                y: 200.,
            },
        ),
    ));
}
pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_walls);
    }
}
