use crate::map::map_core::*;
use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Click, ListenerInput, On, PickHighlight, Pointer, RaycastPickTarget, DragStart, Up},
    selection::PickSelection,
    PickableBundle,
};

#[derive(Event)]
pub struct MapClickEvent(pub ListenerInput<Pointer<Click>>);

// https://github.com/aevyrie/bevy_mod_picking/blame/2ec0793ef747990b710bb08e446bbaf91e2d1e62/examples/event_listener.rs#L94
// TODO: simplify? auto-export all events? (macros?..)
impl From<ListenerInput<Pointer<Click>>> for MapClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        MapClickEvent(event)
    }
}

#[derive(Event)]
pub struct MapUpEvent(pub ListenerInput<Pointer<Up>>);

impl From<ListenerInput<Pointer<Up>>> for MapUpEvent {
    fn from(event: ListenerInput<Pointer<Up>>) -> Self {
        MapUpEvent(event)
    }
}

#[derive(Event)]
pub struct MapDragStartEvent(pub ListenerInput<Pointer<DragStart>>);

impl From<ListenerInput<Pointer<DragStart>>> for MapDragStartEvent {
    fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
        MapDragStartEvent(event)
    }
}

fn setup_pickable(mut commands: Commands, query: Query<Entity, Added<Map>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()))
            .insert(On::<Pointer<Click>>::send_event::<MapClickEvent>())
            .insert(On::<Pointer<DragStart>>::send_event::<MapDragStartEvent>())
            .insert(On::<Pointer<Up>>::send_event::<MapUpEvent>())
            .remove::<(PickSelection, PickHighlight)>();
    }
}

pub struct MapControlsMousePlugin;

impl Plugin for MapControlsMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_pickable)
            .add_event::<MapClickEvent>()
            .add_event::<MapDragStartEvent>()
            .add_event::<MapUpEvent>();
    }
}
