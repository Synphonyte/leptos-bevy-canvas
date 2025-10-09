use bevy::core_pipeline::Skybox;
use bevy::ecs::error::BevyError;
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    commands.spawn((
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.2, 0.0),
            ..default()
        },
        Transform::from_xyz(-0.3, 0.3, 0.2),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
            ..default()
        },
        Skybox {
            image: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            brightness: 500.0,
            rotation: Quat::IDENTITY,
        },
    ));

    Ok(())
}
