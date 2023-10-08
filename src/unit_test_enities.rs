use crate::unit::*;
use crate::unit_move::*;
use bevy::prelude::*;
use std::collections::VecDeque;

fn unit_add(mut commands: Commands) {
    commands.spawn((
        UnitBundle {
            position: Position(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        MaxSpeed(15.),
        Direction(Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }),
        Waypoints(VecDeque::from([
            Vec3 {
                x: 30.0,
                y: 30.0,
                z: 0.,
            },
            Vec3 {
                x: 40.0,
                y: 30.0,
                z: 0.,
            },
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        ])),
    ));
    commands.spawn((
        UnitBundle {
            position: Position(Vec3 {
                x: 10.0,
                y: 30.0,
                z: 20.0,
            }),
            ..Default::default()
        },
        MaxSpeed(15.),
        Direction(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }),
    ));
    commands.spawn((
        UnitBundle {
            position: Position(Vec3 {
                x: -20.0,
                y: 10.0,
                z: 10.0,
            }),
            ..Default::default()
        },
        MaxSpeed(15.),
        Direction(Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    ));
    commands.spawn((
        UnitBundle {
            position: Position(Vec3 {
                x: -10.0,
                y: -50.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        MaxSpeed(15.),
        Direction(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
    ));
}

pub struct UnitTestEntitiesPlugin;

impl Plugin for UnitTestEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, unit_add);
    }
}
