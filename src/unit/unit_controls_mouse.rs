use crate::map::map_controls_mouse::MapClickEvent;
use crate::unit::unit_core::*;
use crate::unit_move::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::collections::VecDeque;

// TODO: create a separate pick mesh?
fn make_unit_pickable(mut commands: Commands, query: Query<Entity, Added<Unit>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()));
    }
}

const WAYPOINT_Z_OFFSET: f32 = 10.;

fn create_waypoints_for_selected(
    mut events: EventReader<MapClickEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &PickSelection, Option<&mut Waypoints>), With<Unit>>,
) {
    for event in events.iter() {
        println!("------------ create_waypoints_for_selected");
        if event.0.button != PointerButton::Secondary {
            return;
        }

        let Some(position) = event.0.hit.position else {
        eprintln!("no position is presented");
        return;
    };

        for (entity, pick_selection, waypoints) in &mut query {
            if !pick_selection.is_selected {
                continue;
            }

            let waypoint_position = Vec3 {
                z: position.z + WAYPOINT_Z_OFFSET,
                ..position
            };

            match waypoints {
                Some(mut w) => w.0.push_back(waypoint_position),
                None => {
                    commands
                        .entity(entity)
                        .insert(Waypoints(VecDeque::from([waypoint_position])));
                }
            }
        }
    }
}

pub struct UnitControlsMousePlugin;

impl Plugin for UnitControlsMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_unit_pickable,
                create_waypoints_for_selected,
            ),
        );
    }
}
