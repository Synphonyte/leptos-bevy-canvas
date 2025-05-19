use crate::bevy_app::components::*;
use bevy::asset::Assets;
use bevy::color::palettes::tailwind::*;
use bevy::color::Color;
use bevy::core_pipeline::Skybox;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, PointLight, StandardMaterial};
use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ecs::error::BevyError;
use bevy::render::render_resource::Face;

const CUBE_X: f32 = 4.0;
const CUBE_Y: f32 = 0.0;
const CUBE_SCALE: f32 = 3.0;
const HIGHLIGHT_SCALE: f32 = 1.03;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> Result<(), BevyError> {
    // Cubes
    let cube = meshes.add(Cuboid::default());

    let highlight_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        cull_mode: Some(Face::Front),
        unlit: true,
        ..default()
    });

    commands
        .spawn((
            Mesh3d(cube.clone()),
            ObjectColor::new(Color::from(RED_500)),
            MeshMaterial3d(materials.add(Color::from(RED_500))),
            Transform::from_xyz(-CUBE_X, CUBE_Y, 0.0).with_scale(Vec3::splat(CUBE_SCALE)),
            RotationSpeed::new(0.01),
        ))
        .observe(select_on_click)
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(highlight_material.clone()),
                    Transform::from_scale(Vec3::splat(HIGHLIGHT_SCALE)),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .observe(select_on_click);
        });

    commands
        .spawn((
            Mesh3d(cube.clone()),
            ObjectColor::new(Color::from(GREEN_500)),
            MeshMaterial3d(materials.add(Color::from(GREEN_500))),
            Transform::from_xyz(CUBE_X, CUBE_Y, 0.0).with_scale(Vec3::splat(CUBE_SCALE)),
            RotationSpeed::new(-0.02),
        ))
        .observe(select_on_click)
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(highlight_material),
                    Transform::from_scale(Vec3::splat(HIGHLIGHT_SCALE)),
                    Pickable::IGNORE,
                    Visibility::Hidden,
                ))
                .observe(select_on_click);
        });

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
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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

pub fn select_on_click(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    prev_selected: Query<Entity, With<Selected>>,
) -> Result<(), BevyError> {
    if let Ok(entity) = prev_selected.single() {
        commands.entity(entity).remove::<Selected>();
    }

    commands.entity(click.target()).insert(Selected);
    Ok(())
}
