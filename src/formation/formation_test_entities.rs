use crate::formation::formation_core::*;
use crate::formation_move::*;
use crate::unit::unit_core::*;
use crate::unit_move::MaxSpeed;
use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

fn formation_add(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let units = (0..15)
        .map(|_| {
            return commands
                .spawn((
                    UnitBundle {
                        position: Position(Vec3 {
                            x: rng.gen_range(-50.0..50.0),
                            y: rng.gen_range(-50.0..50.0),
                            z: 0.,
                        }),
                        ..Default::default()
                    },
                    MaxSpeed(40.),
                ))
                .id();
        })
        .collect::<Vec<Entity>>();

    commands
        .spawn(FormationBundle {
            units: Units(units),
            zone: Zone::new(
                Vec3 {
                    x: 30.,
                    y: 20.,
                    z: 0.,
                },
                50.,
                30.,
                std::f32::consts::PI * 0.05,
            ),
            ..default()
        })
        .insert(NextZonesPath(VecDeque::from([
            NextZone::new(
                Vec3 {
                    x: 0.,
                    y: 50.,
                    z: 0.,
                },
                40.,
                30.,
                std::f32::consts::PI * 0.,
            ),
            NextZone::new(
                Vec3 {
                    x: -30.,
                    y: 40.,
                    z: 0.,
                },
                50.,
                50.,
                std::f32::consts::PI * 1.2,
            ),
        ])));
}

pub struct FormationTestEntitiesPlugin;

impl Plugin for FormationTestEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, formation_add);
    }
}
