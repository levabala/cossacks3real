use crate::map::*;
use crate::unit::*;
use crate::unit_move::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Initialized;

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
    event: Listener<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(Entity, &PickSelection, Option<&mut Waypoints>), With<Unit>>,
) {
    if event.button != PointerButton::Secondary {
        return;
    }

    let Some(position) = event.hit.position else {
        println!("no position is presented");
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

fn add_map_click_listener(
    mut commands: Commands,
    query: Query<Entity, (With<Map>, Without<Initialized>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()))
            .remove::<(PickSelection, PickHighlight)>()
            .insert(On::<Pointer<Click>>::run(create_waypoints_for_selected))
            .insert(Initialized);
    }
}

pub struct UnitControlsMousePlugin;

impl Plugin for UnitControlsMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_unit_pickable, add_map_click_listener));
    }
}
