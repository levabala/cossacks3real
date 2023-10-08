use bevy::prelude::*;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Bundle)]
pub struct UnitBundle {
    pub market: Unit,
    pub position: Position,
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            market: Unit,
            position: Position(default()),
        }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, _app: &mut App) {}
}
