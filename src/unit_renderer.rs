use crate::unit::*;
use bevy::prelude::*;

// TODO: remove the component. use Added<> in query instead
#[derive(Component)]
struct Render;

// TODO: move to separate plugin
fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-50., -50., 300.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 550000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 200.0),
        ..default()
    });
}

// TODO: move to unit definition
const SIZE: f32 = 10.0;

fn draw_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Position), (With<Unit>, Without<Render>)>,
) {
    for (entity, position) in query.iter() {
        let material = materials.add(StandardMaterial {
            base_color: Color::BLUE,
            ..default()
        });

        let mesh = meshes.add(shape::Box::new(SIZE, SIZE, SIZE).into());

        commands.entity(entity).insert(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position.0),
            ..default()
        });
        commands.entity(entity).insert(Render);
    }
}

fn update_unit_drawing(mut query: Query<(&mut Transform, &Position), With<Unit>>) {
    for (mut transform, position) in &mut query {
        *transform = Transform::from_translation(position.0);
    }
}

pub struct UnitRendererPlugin;

impl Plugin for UnitRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, draw_units));
        app.add_systems(Update, (draw_units, update_unit_drawing));
    }
}
