use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Size(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec3);

fn setup(mut commands: Commands) {
    commands.spawn((
        Map,
        Size(Vec2 { x: 300., y: 300. }),
        Position(Vec3 {
            x: 0.,
            y: 0.,
            z: -0.1,
        }),
    ));
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
