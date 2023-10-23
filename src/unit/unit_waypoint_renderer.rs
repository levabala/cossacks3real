use crate::unit::unit_core::*;
use crate::unit_move::*;
use bevy::prelude::*;
use std::collections::VecDeque;

struct WaypointDrawed {
    position: Vec3,
    entity: Entity,
}

#[derive(Component)]
struct WaypointDrawedList(VecDeque<WaypointDrawed>);

const SIZE: f32 = 1.0;

fn create_waypoints_drawed(
    mut commands: Commands,
    mut query: Query<Entity, (With<Unit>, Without<WaypointDrawedList>)>,
) {
    for entity in &mut query {
        commands
            .entity(entity)
            .insert(WaypointDrawedList(VecDeque::new()));
    }
}

fn draw_waypoints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Waypoints, &mut WaypointDrawedList), (With<Unit>, Changed<Waypoints>)>,
) {
    for (waypoint_list, mut waypoint_drawed_list) in &mut query {
        // TODO: compare only the first and the last one
        for waypoint_drawed in waypoint_drawed_list.0.iter() {
            let is_waypoint_deleted = !waypoint_list.0.contains(&waypoint_drawed.position);
            if is_waypoint_deleted {
                commands.entity(waypoint_drawed.entity).despawn();
                waypoint_drawed_list.0.pop_back();
            }
        }

        for waypoint in waypoint_list.0.iter() {
            let is_waypoint_added = waypoint_drawed_list
                .0
                .iter()
                .position(|w| &w.position == waypoint)
                .is_none();

            if is_waypoint_added {
                let waypoint_drawing = commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(
                            shape::UVSphere {
                                radius: SIZE,
                                ..default()
                            }
                            .into(),
                        ),
                        material: materials.add(StandardMaterial {
                            base_color: Color::RED,
                            ..default()
                        }),
                        transform: Transform::from_translation(*waypoint),
                        ..default()
                    })
                    .id();

                waypoint_drawed_list.0.push_back(WaypointDrawed {
                    position: *waypoint,
                    entity: waypoint_drawing,
                });
            }
        }
    }
}

pub struct UnitWaypointRendererPlugin;

impl Plugin for UnitWaypointRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_waypoints_drawed,
                draw_waypoints.after(create_waypoints_drawed),
            ),
        );
    }
}
