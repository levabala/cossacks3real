use crate::unit::unit_core::*;
use crate::unit_move::*;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct WaypointDrawed {
    position: Vec3,
    entity: Entity,
}

#[derive(Component, Debug)]
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
        if let Some(waypoint_first) = waypoint_list.0.get(0) {
            let mut pop_until_index_option: Option<usize> = None;
            for (index, waypoint_drawed) in waypoint_drawed_list.0.iter().enumerate() {
                if &waypoint_drawed.position == waypoint_first {
                    break;
                }

                commands.entity(waypoint_drawed.entity).despawn();
                pop_until_index_option = Some(index);
            }

            if let Some(pop_until_index) = pop_until_index_option {
                waypoint_drawed_list.0.drain(0..=pop_until_index);
            }
        }

        let waypoint_drawed_last_option = waypoint_drawed_list.0.back().map(|r| *r);
        for waypoint in waypoint_list.0.iter().rev() {
            if let Some(waypoint_drawed_last) = waypoint_drawed_last_option {
                if *waypoint == waypoint_drawed_last.position {
                    break;
                }
            }

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
