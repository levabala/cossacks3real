use crate::unit::*;
use bevy::prelude::*;
use std::collections::VecDeque;

fn unit_add(mut commands: Commands) {
    commands.spawn((
        UnitBundle {
            max_speed: MaxSpeed(15.),
            position: Position(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            direction: Direction(Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            }),
            ..Default::default()
        },
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
    commands.spawn(UnitBundle {
        max_speed: MaxSpeed(15.),
        position: Position(Vec3 {
            x: 10.0,
            y: 30.0,
            z: 20.0,
        }),
        direction: Direction(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }),
        ..Default::default()
    });
    commands.spawn(UnitBundle {
        max_speed: MaxSpeed(15.),
        position: Position(Vec3 {
            x: -20.0,
            y: 10.0,
            z: 10.0,
        }),
        direction: Direction(Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
        ..Default::default()
    });
    commands.spawn(UnitBundle {
        max_speed: MaxSpeed(15.),
        position: Position(Vec3 {
            x: -10.0,
            y: -50.0,
            z: 0.0,
        }),
        direction: Direction(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
        ..Default::default()
    });
}

pub struct UnitTestEntitiesPlugin;

impl Plugin for UnitTestEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, unit_add);
    }
}
