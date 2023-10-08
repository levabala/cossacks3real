use bevy::prelude::*;
mod unit;
mod unit_renderer;
mod unit_waypoint_renderer;
mod unit_test_enities;
mod unit_move;
mod mouse_controls;
mod unit_controls_mouse;
mod map;
mod map_renderer;

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
            unit_move::UnitMovePlugin,
            mouse_controls::MouseControls,
            unit_controls_mouse::UnitControlsMouse,
            map::MapPlugin,
            map_renderer::MapRendererPlugin,
        ))
        .run()
}
