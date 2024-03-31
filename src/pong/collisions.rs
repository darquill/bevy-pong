use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use super::{ball::Ball, paddle::Paddle};

#[derive(Component, Debug)]
pub struct Collider {
    pub collision_rect: Aabb2d,
}

impl Collider {
    pub fn new(center: Vec2, size: Vec2) -> Collider {
        Collider {
            collision_rect: Aabb2d::new(center, size / 2.),
        }
    }
}

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
pub struct DebugHelper;

pub struct CollisionsPlugin;

fn check_for_collisions(
    mut ball_query: Query<(&mut Ball, &Transform), With<Ball>>,
    colliders_query: Query<(&Collider, Option<&Paddle>), (With<Collider>, Without<Ball>)>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();

    for (collider, maybe_paddle) in &colliders_query {
        let rect = collider.collision_rect;
        let bounding_circle =
            BoundingCircle::new(ball_transform.translation.truncate(), ball.size / 2.0);
        let collision = collide_with_rect(bounding_circle, rect);

        if let (Some(collision), offset) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball.velocity.x > 0.0,
                Collision::Right => reflect_x = ball.velocity.x < 0.0,
                Collision::Top => reflect_y = ball.velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball.velocity.y > 0.0,
            }

            if reflect_x {
                ball.velocity.x = -ball.velocity.x;

                if maybe_paddle.is_some() {
                    ball.velocity.y += -offset / 50.;
                }
            }

            if reflect_y {
                ball.velocity.y = -ball.velocity.y;
            }
        }
    }
}

fn collide_with_rect(ball: BoundingCircle, rect: Aabb2d) -> (Option<Collision>, f32) {
    if !ball.intersects(&rect) {
        return (None, 0.);
    }

    let closest = rect.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    (Some(side), rect.center().y - ball.center().y)
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions);
    }
}
