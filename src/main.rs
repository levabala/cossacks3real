use bevy::prelude::*;
mod unit;
mod unit2d;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, unit::UnitPlugin, unit2d::Unit2dPlugin))
        .run()
}
