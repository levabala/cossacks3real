use bevy::prelude::*;

#[derive(Component)]
struct Unit;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn add_units(mut commands: Commands) {
    commands.spawn((Unit, Position { x: 1.0, y: 1.0 }));
    commands.spawn((Unit, Position { x: 2.0, y: 1.0 }));
}

#[derive(Resource)]
struct PrintTimer(Timer);

fn print_position_system(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<&Position, With<Unit>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for position in &query {
        println!("position: {} {}", position.x, position.y)
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_units)
            .add_systems(Update, print_position_system);
    }
}
