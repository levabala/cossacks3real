use bevy::prelude::*;
use std::collections::VecDeque;
use crate::unit::unit_core::*;

#[derive(Component)]
pub struct MaxSpeed(pub f32);

#[derive(Component)]
pub struct Direction(pub Vec3);

#[derive(Component)]
pub struct Waypoints(pub VecDeque<Vec3>);

#[derive(Component)]
pub struct Move {
    pub move_path: VecDeque<Vec3>,
    pub velocity: Option<Vec3>,
    distance_prev: f32,
}

fn calc_move_path(_p1: Vec3, p2: Vec3) -> VecDeque<Vec3> {
    return VecDeque::from([p2]);
}

fn unit_move_start(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &Waypoints), (With<Unit>, Without<Move>)>,
) {
    for (entity, position, waypoints) in &mut query {
        commands.entity(entity).insert(Move {
            move_path: calc_move_path(position.0, waypoints.0[0]),
            velocity: None,
            distance_prev: f32::INFINITY,
        });
    }
}

fn unit_move_update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &mut Waypoints, &mut Move, &MaxSpeed), With<Unit>>,
) {
    for (entity, mut position, mut waypoints, mut moving, max_speed) in &mut query {
        if moving.move_path.len() == 0 {
            if waypoints.0.len() == 1 {
                commands.entity(entity).remove::<Move>();
                commands.entity(entity).remove::<Waypoints>();
                continue;
            }

            waypoints.0.pop_front();
            moving.move_path = calc_move_path(position.0, waypoints.0[0]);
        }

        let next_move_target = moving.move_path[0];
        let distance = position.0.distance(next_move_target);

        if distance == 0. || distance >= moving.distance_prev {
            position.0 = next_move_target;
            moving.move_path.pop_front();
            moving.distance_prev = f32::INFINITY;
            moving.velocity = None;
            continue;
        }

        moving.distance_prev = distance;

        let velocity = match moving.velocity {
            None => {
                let direction_vec = (next_move_target - position.0).normalize();
                let velocity_vec = direction_vec * max_speed.0;

                moving.velocity = Some(velocity_vec);
                velocity_vec
            }
            Some(velocity) => velocity,
        };

        let move_vec = velocity * time.delta_seconds();
        position.0 += move_vec;
    }
}

pub struct UnitMovePlugin;

impl Plugin for UnitMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (unit_move_start, unit_move_update));
    }
}
