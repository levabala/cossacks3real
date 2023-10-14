use crate::formation::*;
use crate::unit::*;
use crate::unit_move::MaxSpeed;
use bevy::prelude::*;
use rand::Rng;

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
                    MaxSpeed(15.),
                ))
                .id();
        })
        .collect::<Vec<Entity>>();

    commands.spawn(FormationBundle {
        units: Units(units),
        zone: Zone {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            width: 40.,
            height: 20.,
        },
        ..default()
    });
}

pub struct FormationTestEntitiesPlugin;

impl Plugin for FormationTestEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, formation_add);
    }
}
