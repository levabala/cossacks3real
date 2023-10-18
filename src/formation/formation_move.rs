use crate::formation::formation_core::*;
use crate::unit::unit_core::*;
use bevy::prelude::*;
use std::collections::VecDeque;

// TODO: prevent dublication. can't use Zone because it's treated as component..
pub struct NextZone {
    pub position: Vec3,
    pub vector_base: Vec3,
    pub vector_height: Vec3,
}

impl NextZone {
    pub fn new(position: Vec3, width: f32, height: f32, angle: f32) -> NextZone {
        NextZone {
            position,
            vector_base: Vec3::from((Vec2::from_angle(angle) * width, 0.)),
            vector_height: Vec3::from((
                Vec2::from_angle(angle + std::f32::consts::PI / 2.) * height,
                0.,
            )),
        }
    }
}

#[derive(Component)]
pub struct NextZonesPath(pub VecDeque<NextZone>);

fn move_zone(
    mut commands: Commands,
    mut query_slot: Query<(Entity, &Slots, &mut NextZonesPath), With<Formation>>,
    query_unit: Query<&Position, With<Unit>>,
) {
    for (entity, slots, mut next_zones_path) in &mut query_slot {
        if next_zones_path.0.len() == 0 {
            commands.entity(entity).remove::<NextZonesPath>();
            continue;
        }

        let mut all_units_at_slots = true;
        let mut any_unit_found = false;
        for slot in slots.0.iter() {
            let Some(unit_entity) = slot.unit else { continue; };
            any_unit_found = true;

            let query_unit_result = query_unit.get(unit_entity);

            let Ok(unit_position) = query_unit_result else {
                eprintln!("no linked unit found");
                continue;
            };

            let is_same_position = unit_position.0 == slot.position;
            all_units_at_slots = all_units_at_slots && is_same_position;
        }

        if !all_units_at_slots || !any_unit_found {
            continue;
        }

        println!("all units at their slots. moving to the next zone");

        let next_zone_option = next_zones_path.0.pop_back();
        let Some(next_zone) = next_zone_option else { eprintln!("no next zone found"); continue; };
        commands.entity(entity).remove::<Slots>();
        commands.entity(entity).insert(Zone {
            position: next_zone.position,
            vector_base: next_zone.vector_base,
            vector_height: next_zone.vector_height,
        });
    }
}

pub struct FormationMovePlugin;

impl Plugin for FormationMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_zone);
    }
}
