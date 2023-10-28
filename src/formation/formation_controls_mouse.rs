use std::collections::VecDeque;

use crate::{
    formation::formation_core::*,
    map::map_controls_mouse::{MapDragStartEvent, MapUpEvent},
};
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

#[derive(Component, Debug)]
struct FormationNewZonePreview {
    top_right: Vec3,
}

fn selected_formation_create_preview(
    mut events: EventReader<MapDragStartEvent>,
    mut commands: Commands,
    mut query_select: Query<(&PickSelection, &mut Parent), With<FormationBoxDrawing>>,
    mut query_formation: Query<Entity, With<Formation>>,
) {
    for event in events.iter() {
        if event.0.button != PointerButton::Secondary {
            return;
        }

        let Some(hit_position) = event.0.hit.position else {
            eprintln!("no position is presented");
            return;
        };

        for (pick_selection, parent) in &mut query_select {
            if !pick_selection.is_selected {
                continue;
            }

            let Ok(formation) = query_formation.get_mut(parent.get()) else {
                eprintln!("not found matching formation");
                continue;
            };

            let zone_preview = commands
                .spawn(FormationNewZonePreview {
                    top_right: hit_position,
                })
                .id();
            commands.entity(formation).add_child(zone_preview);
        }
    }
}

const MIN_DISTANCE_TO_BE_ADJUSTED: f32 = 20.;

fn selected_formation_go_to_adjusted(
    mut events: EventReader<MapUpEvent>,
    mut commands: Commands,
    mut query_select: Query<(&PickSelection, &mut Parent), With<FormationBoxDrawing>>,
    mut query_formation: Query<(Entity, &Zone, &Children), With<Formation>>,
    query_new_zone_preview: Query<(Entity, &FormationNewZonePreview)>,
) {
    for event in events.iter() {
        if event.0.button != PointerButton::Secondary {
            return;
        }

        let Some(top_right) = event.0.hit.position else {
            eprintln!("not found matching formation");
            continue;
        };

        for (pick_selection, parent) in &mut query_select {
            if !pick_selection.is_selected {
                continue;
            }

            let Ok(( formation, zone, formation_children )) =
                query_formation.get_mut(parent.get()) else {
                    eprintln!("not found matching formation");
                    continue;
                };
            let Some((new_zone_preview_entity, new_zone_preview)) =
                query_new_zone_preview.iter_many(formation_children).next() else {
                    let next_position = Vec3 {
                        z: zone.position.z,
                        ..top_right
                    };

                    commands
                        .entity(formation)
                        .insert(NextZonesPath(VecDeque::from([NextZone {
                            position: next_position,
                            width: zone.width,
                            height: zone.height,
                            direction: zone.direction,
                        }])));
                    continue;
                };

            let area = zone.width * zone.height;
            let distance = Vec2::new(
                top_right.x - new_zone_preview.top_right.x,
                top_right.y - new_zone_preview.top_right.y,
            );
            let width = distance.length();
            let height = area / width;

            let distance_3d = Vec3 {
                x: distance.x,
                y: distance.y,
                z: 0.,
            };

            if width < MIN_DISTANCE_TO_BE_ADJUSTED {
                let next_position = Vec3 {
                    z: zone.position.z,
                    ..(top_right + distance_3d / 2.)
                };

                commands
                    .entity(formation)
                    .insert(NextZonesPath(VecDeque::from([NextZone {
                        position: next_position,
                        width: zone.width,
                        height: zone.height,
                        direction: zone.direction,
                    }])));
                continue;
            }

            let distance_norm = distance.normalize();
            let direction = Vec2 {
                x: -distance_norm.y,
                y: distance_norm.x,
            };

            let direction_3d = Vec3 {
                x: direction.x,
                y: direction.y,
                z: 0.,
            };
            let position = Vec3 {
                z: 0.,
                ..(new_zone_preview.top_right + distance_3d / 2. - direction_3d * height / 2.)
            };

            commands
                .entity(formation)
                .insert(NextZonesPath(VecDeque::from([NextZone {
                    position,
                    width,
                    height,
                    direction,
                }])));

            commands.entity(new_zone_preview_entity).despawn();
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
                selected_formation_go_to_adjusted,
                selected_formation_create_preview,
            ),
        );
    }
}
