use crate::formation::formation_core::*;
use crate::unit::unit_core::*;
use bevy::prelude::*;
use std::collections::VecDeque;

// TODO: prevent dublication. can't use Zone because it's treated as component.. (just wrap it?)
pub struct NextZone {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
    pub direction: Vec2,
}

impl NextZone {
    pub fn new(position: Vec3, width: f32, height: f32, angle: f32) -> NextZone {
        NextZone {
            position,
            width,
            height,
            direction: Vec2::from_angle(angle),
        }
    }
}

#[derive(Component)]
pub struct NextZonesPath(pub VecDeque<NextZone>);

fn move_zone(
    mut commands: Commands,
    mut query_formation: Query<(Entity, &Children, &mut NextZonesPath, &mut Zone), With<Formation>>,
    query_unit: Query<&Position, With<Unit>>,
    query_slot: Query<(Entity, &Slot)>,
) {
    for (entity, slot_entities, mut next_zones_path, mut zone) in &mut query_formation {
        if next_zones_path.0.len() == 0 {
            commands.entity(entity).remove::<NextZonesPath>();
            continue;
        }

        let mut all_units_at_slots = true;
        let mut any_unit_found = false;
        for (_, slot) in query_slot.iter_many(slot_entities) {
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

        for slot_tuple in query_slot.iter_many(slot_entities) {
            commands.entity(slot_tuple.0).despawn_recursive();
        }

        zone.position = next_zone.position;
        zone.width = next_zone.width;
        zone.height = next_zone.height;
        zone.direction = next_zone.direction;
    }
}

pub struct FormationMovePlugin;

impl Plugin for FormationMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_zone);
    }
}
