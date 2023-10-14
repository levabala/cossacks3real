use bevy::prelude::*;
mod scene_setup;
mod unit;
mod unit_renderer;
mod unit_waypoint_renderer;
mod unit_test_enities;
mod unit_move;
mod mouse_controls;
mod unit_controls_mouse;
mod map;
mod map_renderer;
mod camera;

fn main() {
    App::new()
        .add_systems(Startup, || {
            println!("start\n");
        })
        .add_plugins((
            DefaultPlugins,
            scene_setup::SceneSetupPlugin,
            unit::UnitPlugin,
            unit_renderer::UnitRendererPlugin,
            unit_waypoint_renderer::UnitWaypointRendererPlugin,
            unit_test_enities::UnitTestEntitiesPlugin,
            unit_move::UnitMovePlugin,
            mouse_controls::MouseControls,
            unit_controls_mouse::UnitControlsMouse,
            map::MapPlugin,
            map_renderer::MapRendererPlugin,
            camera::CameraPlugin,
        ))
        .run()
}
