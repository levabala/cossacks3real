use bevy::prelude::*;
mod unit;
mod unit_renderer;
mod unit_waypoint_renderer;

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
        ))
        .run()
}
