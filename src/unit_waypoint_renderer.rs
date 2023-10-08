use crate::unit::*;
use crate::unit_move::*;
use bevy::prelude::*;

#[derive(Component)]
struct Render;

const SIZE: f32 = 2.0;

fn draw_waypoints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Waypoints), (With<Unit>, Without<Render>)>,
) {
    for (entity, waypoints) in query.iter() {
        // TODO: remove .cloned()
        for waypoint in waypoints.0.iter().cloned() {
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
            commands.entity(entity).insert(Render);
        }
    }
}

pub struct UnitWaypointRendererPlugin;

impl Plugin for UnitWaypointRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_waypoints);
    }
}
