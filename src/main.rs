use bevy::prelude::*;

mod camera;
mod mouse_controls;
mod scene_setup;
mod axis_render;

mod formation;
use formation::*;

mod map;
use map::*;

mod unit;
use unit::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins((
            scene_setup::SceneSetupPlugin,
            mouse_controls::MouseControlsPlugin,
            camera::CameraPlugin,
            axis_render::AxisRendererPlugin,
        ))
        .add_plugins((map_core::MapPlugin, map_renderer::MapRendererPlugin))
        .add_plugins((
            unit_core::UnitPlugin,
            unit_renderer::UnitRendererPlugin,
            unit_waypoint_renderer::UnitWaypointRendererPlugin,
            unit_test_enities::UnitTestEntitiesPlugin,
            unit_move::UnitMovePlugin,
            unit_controls_mouse::UnitControlsMousePlugin,
        ))
        .add_plugins((
            formation_core::FormationPlugin,
            formation_test_entities::FormationTestEntitiesPlugin,
            formation_renderer::FormationRendererPlugin,
            formation_controls_keyboard::FormationControlsKeyboardPlugin,
            formation_controls_mouse::FormationControlsMousePlugin,
        ))
        .run()
}
