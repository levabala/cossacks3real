use crate::unit::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Render;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const SIZE: f32 = 20.0;

fn draw_units(
    mut commands: Commands,
    query: Query<(Entity, &Position), (With<Unit>, Without<Render>)>,
) {
    for (entity, position) in query.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(SIZE, SIZE)),
                ..default()
            },
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

pub struct UnitRenderer2dPlugin;

impl Plugin for UnitRenderer2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, draw_units));
        app.add_systems(Update, (draw_units, update_unit_drawing));
    }
}
