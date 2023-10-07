use bevy::prelude::*;
mod unit;
mod unit_renderer_2d;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            unit::UnitPlugin,
            unit_renderer_2d::UnitRenderer2dPlugin,
        ))
        .run()
}
