use bevy::prelude::*;
use bevy_polyline::prelude::*;

const LENGTH: f32 = 10.0;

fn draw_axises(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: LENGTH,
                    y: 0.,
                    z: 0.,
                },
            ],
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 1.0,
            color: Color::RED,
            perspective: false,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: 0.,
                    y: LENGTH,
                    z: 0.,
                },
            ],
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 1.0,
            color: Color::GREEN,
            perspective: false,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: LENGTH,
                },
            ],
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 1.0,
            color: Color::BLUE,
            perspective: false,
            ..default()
        }),
        ..default()
    });
}

pub struct AxisRendererPlugin;

impl Plugin for AxisRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_axises);
    }
}
