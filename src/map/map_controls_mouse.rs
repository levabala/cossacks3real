use crate::map::map_core::*;
use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Click, ListenerInput, On, PickHighlight, Pointer, RaycastPickTarget, DragStart, Up, Drag},
    selection::PickSelection,
    PickableBundle,
};

macro_rules! generate_event_struct {
    ($struct_name:ident, $event_type:ty) => {
        #[derive(Event)]
        pub struct $struct_name(pub ListenerInput<Pointer<$event_type>>);

        impl From<ListenerInput<Pointer<$event_type>>> for $struct_name {
            fn from(event: ListenerInput<Pointer<$event_type>>) -> Self {
                $struct_name(event)
            }
        }
    };
}

// TODO: i want to call it like `generate_event_struct!(Click)` (how to prefix $struct_name?)
generate_event_struct!(MapClickEvent, Click);
generate_event_struct!(MapUpEvent, Up);
generate_event_struct!(MapDragStartEvent, DragStart);
generate_event_struct!(MapDragEvent, Drag);

fn setup_pickable(mut commands: Commands, query: Query<Entity, Added<Map>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()))
            .insert(On::<Pointer<Click>>::send_event::<MapClickEvent>())
            .insert(On::<Pointer<DragStart>>::send_event::<MapDragStartEvent>())
            .insert(On::<Pointer<Drag>>::send_event::<MapDragEvent>())
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
            .add_event::<MapDragEvent>()
            .add_event::<MapUpEvent>();
    }
}
