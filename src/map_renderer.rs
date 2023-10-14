use crate::map::*;
use bevy::prelude::*;

const MAP_HEIGHT: f32 = 0.;

fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Position, &Size), Added<Map>>,
) {
    for (entity, position, size) in query.iter() {
        let material = materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });

        let mesh = meshes.add(shape::Box::new(size.0.x, size.0.y, MAP_HEIGHT).into());

        commands.entity(entity).insert(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position.0),
            ..default()
        });
    }
}

pub struct MapRendererPlugin;

impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_map);
    }
}
