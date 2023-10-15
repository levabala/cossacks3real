use bevy::prelude::*;
mod camera;
mod formation;
mod formation_controls_keyboard;
mod formation_controls_mouse;
mod formation_renderer;
mod formation_test_entities;
mod map;
mod map_renderer;
mod mouse_controls;
mod scene_setup;
mod unit;
mod unit_controls_mouse;
mod unit_move;
mod unit_renderer;
mod unit_test_enities;
mod unit_waypoint_renderer;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins((
            scene_setup::SceneSetupPlugin,
            mouse_controls::MouseControlsPlugin,
            camera::CameraPlugin,
        ))
        .add_plugins((map::MapPlugin, map_renderer::MapRendererPlugin))
        .add_plugins((
            unit::UnitPlugin,
            unit_renderer::UnitRendererPlugin,
            unit_waypoint_renderer::UnitWaypointRendererPlugin,
            unit_test_enities::UnitTestEntitiesPlugin,
            unit_move::UnitMovePlugin,
            unit_controls_mouse::UnitControlsMousePlugin,
        ))
        .add_plugins((
            formation::FormationPlugin,
            formation_test_entities::FormationTestEntitiesPlugin,
            formation_renderer::FormationRendererPlugin,
            formation_controls_keyboard::FormationControlsKeyboardPlugin,
            formation_controls_mouse::FormationControlsMousePlugin,
        ))
        .run()
}
