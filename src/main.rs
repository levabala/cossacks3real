use bevy::prelude::*;
mod unit;
mod unit_renderer;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            unit::UnitPlugin,
            unit_renderer::UnitRendererPlugin,
        ))
        .run()
}
