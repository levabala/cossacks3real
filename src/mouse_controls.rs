use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn add_camera_raycast(
    mut commands: Commands,
    query: Query<Entity, Added<Camera>>, // TODO: why can't i check for Camera3dBundle?
) {
    for entity in query.iter() {
        commands.entity(entity).insert(RaycastPickCamera::default());
    }
}

pub struct MouseControls;

impl Plugin for MouseControls {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(Update, add_camera_raycast);
    }
}
