use crate::formation::formation_core::*;
use bevy::prelude::*;
use bevy_polyline::prelude::*;

fn generate_zone_vertices(zone: &Zone) -> Vec<Vec3> {
    let vec_vertical = Vec3 {
        x: 0.,
        y: zone.height / 2.,
        z: 0.,
    };
    let vec_horizontal = Vec3 {
        x: zone.width / 2.,
        y: 0.,
        z: 0.,
    };
    let verticies: [Vec3; 5] = [
        -vec_vertical - vec_horizontal,
        -vec_vertical + vec_horizontal,
        vec_vertical + vec_horizontal,
        vec_vertical - vec_horizontal,
        -vec_vertical - vec_horizontal,
    ];

    return Vec::from(verticies);
}

fn calc_formation_translation(zone: &Zone) -> Transform {
    let mut transform = Transform::from_translation(zone.position);
    transform.rotate_z(-zone.direction.angle_between(Vec2::Y));

    return transform;
}

fn setup_transform_formation(
    mut commands: Commands,
    query: Query<(Entity, &Zone), (With<Formation>, Added<Zone>)>,
) {
    for (entity, zone) in query.iter() {
        commands.entity(entity).insert(PbrBundle {
            transform: calc_formation_translation(zone),
            ..default()
        });
    }
}

fn update_transform_formation(
    mut commands: Commands,
    query: Query<(Entity, &Zone), (With<Formation>, Changed<Zone>)>,
) {
    for (entity, zone) in query.iter() {
        commands
            .entity(entity)
            .insert(calc_formation_translation(zone));
    }
}

fn create_formation_outline(
    polyline_materials: &mut ResMut<Assets<PolylineMaterial>>,
    polylines: &mut ResMut<Assets<Polyline>>,
    zone: &Zone,
) -> PolylineBundle {
    return PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: generate_zone_vertices(zone),
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 1.0,
            color: Color::RED,
            perspective: false,
            ..default()
        }),
        ..default()
    };
}

// TODO: use regular mesh?
fn draw_formation_outline(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<(Entity, &Zone), Added<Formation>>,
) {
    for (entity, zone) in query.iter() {
        let outline = commands
            .spawn(create_formation_outline(
                &mut polyline_materials,
                &mut polylines,
                zone,
            ))
            .id();

        commands.entity(entity).add_child(outline);
    }
}

// TODO: modify instead of despawn-respawn
fn update_formation_outline(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<(Entity, &Zone, &Children), (With<Formation>, Changed<Zone>)>,
    query_outline: Query<Entity, With<Handle<Polyline>>>,
) {
    for (entity, zone, children) in query.iter() {
        for outline_outdated in query_outline.iter_many(children) {
            commands.entity(outline_outdated).despawn();

            let outline_new = commands
                .spawn(create_formation_outline(
                    &mut polyline_materials,
                    &mut polylines,
                    zone,
                ))
                .id();
            commands.entity(entity).add_child(outline_new);
        }
    }
}

#[derive(Component)]
pub struct Drawing(Entity);

fn draw_formation_slots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Slots), Added<Slots>>,
) {
    for (entity, slots) in query.iter() {
        for slot in &slots.0 {
            let drawing = commands
                .spawn(PbrBundle {
                    mesh: meshes.add(shape::Circle::new(2.5).into()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::YELLOW,
                        ..default()
                    }),
                    transform: Transform::from_translation(slot.position),
                    ..default()
                })
                .id();

            let drawing_linking = commands.spawn(Drawing(drawing)).id();

            commands.entity(entity).add_child(drawing_linking);
        }
    }
}

// TODO: do not remove. just move them duh
fn handle_formation_slots_removal(
    mut commands: Commands,
    mut removals: RemovedComponents<Slots>,
    query_children: Query<&Children>,
    query_drawing: Query<Entity, With<Drawing>>,
) {
    for slots in &mut removals {
        let Ok(drawings) = query_children.get(slots) else {
            eprintln!("matching drawings not found");
            continue;
        };

        for drawing_entity in query_drawing.iter_many(drawings) {
            commands.entity(slots).remove_children(&[drawing_entity]);
            commands.entity(drawing_entity).despawn_recursive();
        }
    }
}

pub struct FormationRendererPlugin;

impl Plugin for FormationRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PolylinePlugin)
            .add_systems(
                Update,
                (
                    setup_transform_formation,
                    update_transform_formation,
                    draw_formation_outline,
                    draw_formation_slots,
                    update_formation_outline,
                ),
            )
            .add_systems(PostUpdate, handle_formation_slots_removal);
    }
}
