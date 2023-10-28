use crate::map::map_core::*;
use bevy::prelude::*;

const MAP_HEIGHT: f32 = 0.;
const MAP_SURFACE_OFFSET: f32 = -0.01;

fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Size), Added<Map>>,
) {
    for (entity, size) in query.iter() {
        let material = materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });

        let drawing = commands
            .spawn(PbrBundle {
                mesh: meshes.add(shape::Box::new(size.0.x, size.0.y, MAP_HEIGHT).into()),
                material,
                transform: Transform::from_translation(Vec3 {
                    x: 0.,
                    y: 0.,
                    z: MAP_SURFACE_OFFSET,
                }),
                ..default()
            })
            .id();

        commands
            .entity(entity)
            .insert(PbrBundle {
                mesh: meshes.add(shape::Box::new(size.0.x, size.0.y, 0.).into()),
                ..default()
            })
            .remove::<Handle<StandardMaterial>>()
            .add_child(drawing);
    }
}

pub struct MapRendererPlugin;

impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_map);
    }
}
