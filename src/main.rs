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
mod formation;
mod formation_test_entities;
mod formation_renderer;
mod camera;

fn main() {
    App::new()
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
            formation::FormationPlugin,
            formation_test_entities::FormationTestEntitiesPlugin,
            formation_renderer::FormationRendererPlugin,
            camera::CameraPlugin,
        ))
        .run()
}
