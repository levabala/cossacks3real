use bevy::prelude::*;
mod unit;
mod unit_renderer;
mod unit_waypoint_renderer;
mod unit_test_enities;

fn main() {
    App::new()
        .add_systems(Startup, || {
            println!("start\n");
        })
        .add_plugins((
            DefaultPlugins,
            unit::UnitPlugin,
            unit_renderer::UnitRendererPlugin,
            unit_waypoint_renderer::UnitWaypointRendererPlugin,
            unit_test_enities::UnitTestEntitiesPlugin,
        ))
        .run()
}
