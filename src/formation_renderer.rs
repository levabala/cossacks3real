use crate::formation::*;
use bevy::prelude::*;
use bevy_polyline::prelude::*;

fn generate_zone_vertices(zone: &Zone) -> Vec<Vec3> {
    let verticies: [Vec3; 5] = [
        zone.position - (zone.vector_base / 2.) - (zone.vector_height / 2.),
        zone.position - (zone.vector_base / 2.) + (zone.vector_height / 2.),
        zone.position + (zone.vector_base / 2.) + (zone.vector_height / 2.),
        zone.position + (zone.vector_base / 2.) - (zone.vector_height / 2.),
        zone.position - (zone.vector_base / 2.) - (zone.vector_height / 2.),
    ];

    return Vec::from(verticies);
}

// TODO: use regular mesh?
fn draw_formation_outline(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<&Zone, Added<Formation>>,
) {
    for zone in query.iter() {
        commands.spawn(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: generate_zone_vertices(zone),
            }),
            material: polyline_materials.add(PolylineMaterial {
                width: 1.0,
                color: Color::RED,
                perspective: false,
                ..default()
            }),
            ..default()
        });
    }
}

fn draw_formation_slots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Slots, Added<Slots>>,
) {
    for slots in query.iter() {
        for slot in &slots.0 {
            commands.spawn(PbrBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()),
                material: materials.add(StandardMaterial {
                    base_color: Color::YELLOW,
                    ..default()
                }),
                transform: Transform::from_translation(slot.position),
                ..default()
            });
        }
    }
}

pub struct FormationRendererPlugin;

impl Plugin for FormationRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PolylinePlugin)
            .add_systems(Update, (draw_formation_outline, draw_formation_slots));
    }
}
