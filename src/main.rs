use bevy::prelude::*;
mod unit;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, unit::UnitPlugin))
        .run()
}
