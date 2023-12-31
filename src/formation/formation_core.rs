use crate::unit_core::*;
use crate::unit_move::*;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Formation;

#[derive(Component)]
pub struct Zone {
    pub position: Vec3,
    pub width: f32,
    pub height: f32,
    pub direction: Vec2,
}

impl Zone {
    pub fn new(position: Vec3, width: f32, height: f32, angle: f32) -> Zone {
        Zone {
            position,
            width,
            height,
            direction: Vec2::from_angle(angle),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Slot {
    pub position: Vec3,
    pub position_relative: Vec3,
    pub unit: Option<Entity>,
}

#[derive(Component)]
pub struct SlotsParams {
    pub rows: u32,
    pub cols: u32,
}

#[derive(Component)]
pub struct Units(pub Vec<Entity>); // TODO: consider adding optional Slot reference

#[derive(Bundle)]
pub struct FormationBundle {
    pub marker: Formation,
    pub zone: Zone,
    pub units: Units,
    pub slots_params: SlotsParams,
}

impl Default for FormationBundle {
    fn default() -> Self {
        Self {
            marker: Formation,
            zone: Zone::new(
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                20.,
                10.,
                0.,
            ),
            units: Units(default()),
            slots_params: SlotsParams { rows: 3, cols: 4 },
        }
    }
}

fn generate_slots(
    mut commands: Commands,
    query: Query<(Entity, &Zone, &SlotsParams), (With<Formation>, Changed<Zone>)>,
) {
    for (entity, zone, slots_params) in query.iter() {
        let vector_vertical = zone.direction * zone.height;
        let vector_horizontal = Vec2 {
            x: -zone.direction.y,
            y: zone.direction.x,
        } * zone.width;

        for index_row in 0..slots_params.rows {
            let vec_height = vector_vertical / (slots_params.rows - 1) as f32 * index_row as f32
                - vector_vertical / 2.;
            let y_relative =
                zone.height / (slots_params.rows - 1) as f32 * index_row as f32 - zone.height / 2.;

            for index_col in 0..slots_params.cols {
                let vec_width = vector_horizontal / (slots_params.cols - 1) as f32
                    * index_col as f32
                    - vector_horizontal / 2.;
                let x_relative = zone.width / (slots_params.cols - 1) as f32 * index_col as f32
                    - zone.width / 2.;

                let vec_delta = vec_height + vec_width;
                let x = zone.position.x + vec_delta.x;
                let y = zone.position.y + vec_delta.y;
                let z = zone.position.z;

                let vec_relative = Vec3 {
                    x: x_relative,
                    y: y_relative,
                    z: 0.,
                };

                let slot = commands
                    .spawn(Slot {
                        position: Vec3 { x, y, z },
                        position_relative: vec_relative,
                        unit: None,
                    })
                    .id();
                commands.entity(entity).add_child(slot);
            }
        }
    }
}

fn assign_units(
    mut query: Query<
        (&Units, &Children),
        (With<Formation>, Or<(Changed<Units>, Changed<Children>)>),
    >,
    mut query_slot: Query<&mut Slot>,
) {
    for (units, children) in &mut query {
        let mut units_not_assigned: Vec<&Entity> = Vec::new();
        for unit in units.0.iter() {
            let mut slot_found = false;
            for slot in query_slot.iter_many(children) {
                if Some(unit) == slot.unit.as_ref() {
                    slot_found = true;
                    break;
                }
            }

            if slot_found {
                continue;
            }

            units_not_assigned.push(unit);
        }

        let units_not_assigned_len = units_not_assigned.len();

        let mut slots_iter_mut = query_slot.iter_many_mut(children);
        let mut slot_index = 0;
        while let Some(mut slot) = slots_iter_mut.fetch_next() {
            if slot_index >= units_not_assigned_len {
                break;
            }

            slot.unit = Some(*units_not_assigned[slot_index]);

            slot_index += 1;
        }
    }
}

fn control_assigned_units(
    mut commands: Commands,
    query_slot: Query<&Slot, Changed<Slot>>,
    query_unit: Query<(&Position, Option<&Waypoints>), With<Unit>>,
) {
    for slot in query_slot.iter() {
        let Some(unit_entity) = slot.unit else { continue; };

        let query_unit_result = query_unit.get(unit_entity);
        match query_unit_result {
            // TODO: check if unit waypoints are already correct?
            Ok((unit_position, _unit_waypoints)) => {
                let is_same_position = unit_position.0 == slot.position;

                if !is_same_position {
                    commands
                        .entity(unit_entity)
                        .insert(Waypoints(VecDeque::from([slot.position])));
                }
            }
            Err(e) => {
                eprintln!("failed to acquire unit position: {:?}", e);
            }
        }
    }
}

pub struct FormationPlugin;

impl Plugin for FormationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (generate_slots, assign_units, control_assigned_units),
        );
    }
}
