use bevy::core_pipeline::Skybox;
use bevy::ecs::error::BevyError;
use bevy::light::PointLight;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::bevy_app::resources::AssetsLoading;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
) -> Result<(), BevyError> {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Orbital Camera
    let diffuse_map = asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2");
    let specular_map = asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2");

    assets_loading.insert(diffuse_map.id().into());
    assets_loading.insert(specular_map.id().into());

    commands.spawn((
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.2, 0.0),
            ..default()
        },
        Transform::from_xyz(-0.3, 0.3, 0.2),
        Skybox {
            image: diffuse_map.clone(),
            brightness: 500.0,
            rotation: Quat::IDENTITY,
        },
        EnvironmentMapLight {
            diffuse_map,
            specular_map,
            intensity: 900.0,
            ..default()
        },
    ));

    Ok(())
}
