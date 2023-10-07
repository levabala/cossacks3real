use bevy::prelude::*;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Speed(pub Vec3);

#[derive(Component)]
pub struct Direction(pub Vec3);

#[derive(Bundle)]
pub struct UnitBundle {
    market: Unit,
    position: Position,
    speed: Speed,
    direction: Direction,
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            market: Unit,
            position: Position(Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }),
            speed: Speed(Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }),
            direction: Direction(Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            }),
        }
    }
}

fn unit_add(mut commands: Commands) {
    let units_array = [
        UnitBundle {
            position: Position(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            direction: Direction(Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        UnitBundle {
            position: Position(Vec3 {
                x: 10.0,
                y: 30.0,
                z: 20.0,
            }),
            direction: Direction(Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        UnitBundle {
            position: Position(Vec3 {
                x: -20.0,
                y: 10.0,
                z: 10.0,
            }),
            direction: Direction(Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        UnitBundle {
            position: Position(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            direction: Direction(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        },
    ];

    for unit in units_array {
        commands.spawn(unit);
    }
}

fn unit_move(time: Res<Time>, mut query: Query<(&mut Position, &Direction), With<Unit>>) {
    const SPEED: f32 = 30.;

    for (mut position, direction) in &mut query {
        let delta_scale = SPEED * time.delta_seconds();
        let delta_vec = delta_scale * direction.0;

        position.0 += delta_vec;
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, unit_add)
            .add_systems(Update, unit_move);
    }
}
