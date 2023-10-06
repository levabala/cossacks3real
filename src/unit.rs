use bevy::prelude::*;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

fn unit_add(mut commands: Commands) {
    let units_array = [
        (Unit, Position { x: 0.0, y: 0.0 }, Direction::Up),
        (Unit, Position { x: 10.0, y: 1.0 }, Direction::Right),
        (Unit, Position { x: 20.0, y: 30.0 }, Direction::Right),
        (Unit, Position { x: 30.0, y: 20.0 }, Direction::None),
    ];

    for unit in units_array {
        commands.spawn(unit);
    }
}

#[derive(Resource)]
struct PrintTimer(Timer);

fn unit_move(time: Res<Time>, mut query: Query<(&mut Position, &Direction), With<Unit>>) {
    const SPEED: f32 = 30.;

    for (mut position, direction) in &mut query {
        let delta = SPEED * time.delta_seconds();
        match direction {
            Direction::Up => position.y += delta,
            Direction::Down => position.y -= delta,
            Direction::Right => position.x += delta,
            Direction::Left => position.x -= delta,
            Direction::None => (),
        }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, unit_add)
            .add_systems(Update, unit_move);
    }
}
