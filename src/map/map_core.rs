use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Size(pub Vec2);

// TODO: move to separate file
fn setup(mut commands: Commands) {
    commands.spawn((
        Map,
        Size(Vec2 { x: 300., y: 300. }),
    ));
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}
