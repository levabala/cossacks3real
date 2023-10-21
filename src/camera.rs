use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_translate_sensitivity: Vec2::splat(10.),
                ..default()
            },
            Vec3::new(-50., -100., 300.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin)
            .add_plugins(OrbitCameraPlugin::default())
            .add_systems(Startup, setup);
    }
}
