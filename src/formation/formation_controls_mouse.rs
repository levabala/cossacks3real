use std::collections::VecDeque;

use crate::{formation::formation_core::*, map::map_controls_mouse::MapClickEvent};
use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::prelude::*;

use super::formation_move::{NextZone, NextZonesPath};

#[derive(Component)]
pub struct Initialized;

const FORMATION_HEIGHT: f32 = 10.;

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Vec4::new(0.0, 0.0, 0.0, 0.05),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Vec4::new(0.0, 0.0, 0.0, 0.11),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Vec4::new(0.0, 0.0, 0.0, 0.1),
        ..matl.to_owned()
    })),
};

#[derive(Component)]
pub struct FormationBoxDrawing;

fn draw_formation_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Zone, Option<&Children>), (With<Formation>, Changed<Zone>)>,
    query_formation_box: Query<Entity, With<FormationBoxDrawing>>,
) {
    for (entity, zone, children_option) in query.iter() {
        match children_option {
            Some(children) => {
                for pick_box in query_formation_box.iter_many(children) {
                    commands
                        .entity(pick_box)
                        .insert(meshes.add(
                            shape::Box::new(zone.width, zone.height, FORMATION_HEIGHT).into(),
                        ));
                }
            }
            None => {
                let pick_box = commands
                    .spawn(FormationBoxDrawing)
                    .insert(PbrBundle {
                        mesh: meshes
                            .add(shape::Box::new(zone.width, zone.height, FORMATION_HEIGHT).into()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgba(1., 1., 0., 0.05),
                            alpha_mode: AlphaMode::Blend,
                            unlit: true,
                            ..default()
                        }),
                        ..default()
                    })
                    .insert(NotShadowCaster)
                    .insert(HIGHLIGHT_TINT.clone())
                    .insert((PickableBundle::default(), RaycastPickTarget::default()))
                    .id();

                commands.entity(entity).add_child(pick_box);
            }
        }
    }
}

fn make_formation_pickable(mut commands: Commands, query: Query<Entity, Added<Formation>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()));
    }
}

fn selected_formation_go_to(
    mut events: EventReader<MapClickEvent>,
    mut commands: Commands,
    mut query_select: Query<(&PickSelection, &mut Parent), With<FormationBoxDrawing>>,
    mut query_formation: Query<(Entity, &Zone), With<Formation>>,
) {
    for event in events.iter() {
        if event.0.button != PointerButton::Secondary {
            return;
        }

        let Some(position) = event.0.hit.position else {
            eprintln!("no position is presented");
            return;
        };

        for (pick_selection, parent) in &mut query_select {
            if !pick_selection.is_selected {
                continue;
            }

            let Ok(( formation, zone )) = query_formation.get_mut(parent.get()) else {
                eprintln!("not found matching formation");
                continue;
            };

            let next_position = Vec3 {
                z: zone.position.z,
                ..position
            };

            commands
                .entity(formation)
                .insert(NextZonesPath(VecDeque::from([NextZone {
                    position: next_position,
                    width: zone.width,
                    height: zone.height,
                    direction: zone.direction,
                }])));
        }
    }
}

pub struct FormationControlsMousePlugin;

impl Plugin for FormationControlsMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_formation_box,
                make_formation_pickable,
                selected_formation_go_to,
            ),
        );
    }
}
