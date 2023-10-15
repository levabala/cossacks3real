use crate::formation::*;
use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::prelude::*;

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

fn create_formation_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Zone), Added<Formation>>,
) {
    for (entity, zone) in query.iter() {
        let mut transform = Transform::from_translation(zone.position);
        transform.rotate_z(zone.vector_base.angle_between(Vec3::X));

        commands
            .entity(entity)
            .insert(PbrBundle {
                mesh: meshes.add(
                    shape::Box::new(
                        zone.vector_base.length(),
                        zone.vector_height.length(),
                        FORMATION_HEIGHT,
                    )
                    .into(),
                ),
                transform,
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
            // .remove::<Handle<StandardMaterial>>()
            .insert((PickableBundle::default(), RaycastPickTarget::default()));
    }
}

fn make_formation_pickable(mut commands: Commands, query: Query<Entity, Added<Formation>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((PickableBundle::default(), RaycastPickTarget::default()))
            .insert(On::<Pointer<Click>>::run(|| {
                println!("formation click");
            }));
    }
}

pub struct FormationControlsMousePlugin;

impl Plugin for FormationControlsMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_formation_box, make_formation_pickable));
    }
}