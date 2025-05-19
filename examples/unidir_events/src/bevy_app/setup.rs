use bevy::asset::Assets;
use bevy::color::palettes::tailwind::GRAY_700;
use bevy::color::Color;
use bevy::core_pipeline::Skybox;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, PointLight, StandardMaterial};
use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::render::mesh::CylinderMeshBuilder;
use bevy::ecs::error::BevyError;

pub const CAMERA_LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, -0.2);

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> Result<(), BevyError> {
    let ground_matl = materials.add(Color::from(GRAY_700));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(CylinderMeshBuilder::new(7.0, 10.0, 16).build())),
        MeshMaterial3d(ground_matl.clone()),
        Pickable::IGNORE, // Disable picking for the ground plane.
        Transform::from_xyz(0.0, -5.5, CAMERA_LOOK_AT.z),
        Name::new("Ground"),
    ));

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

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(CAMERA_LOOK_AT, Vec3::Y),
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
