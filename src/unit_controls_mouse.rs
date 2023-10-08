use crate::unit::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn update_unit_drawing(mut commands: Commands, query: Query<Entity, Added<Unit>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()));
    }
}

pub struct UnitControlsMouse;

impl Plugin for UnitControlsMouse {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_unit_drawing);
    }
}
