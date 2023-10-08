use crate::unit::*;
use crate::unit_move::*;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
struct WaypointsDrawed(VecDeque<Vec3>);

const SIZE: f32 = 2.0;

fn create_waypoints_drawed(
    mut commands: Commands,
    mut query: Query<Entity, (With<Unit>, Without<WaypointsDrawed>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(WaypointsDrawed(VecDeque::new()));
    }
}

fn draw_waypoints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Waypoints, &mut WaypointsDrawed), With<Unit>>,
) {
    for (waypoints, mut waypoints_drawed) in &mut query {
        // TODO: remove .cloned()
        for waypoint in waypoints.0.iter().cloned() {
            match waypoints_drawed.0.iter().position(|w| *w == waypoint) {
                Some(_) => continue,
                None => {
                    waypoints_drawed.0.push_back(waypoint)
                }
            }

            let material = materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            });

            let mesh = meshes.add(
                shape::UVSphere {
                    radius: SIZE,
                    ..default()
                }
                .into(),
            );

            commands.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(waypoint),
                ..default()
            });
        }
    }
}

pub struct UnitWaypointRendererPlugin;

impl Plugin for UnitWaypointRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_waypoints_drawed, draw_waypoints));
    }
}
