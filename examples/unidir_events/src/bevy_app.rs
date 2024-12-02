use crate::events::TextEvent;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::asset::{AssetMetaCheck, RenderAssetUsages};
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::WindowResolution;
use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction};
use leptos_bevy_canvas::prelude::*;
use leptos_bevy_canvas_examples::camera_from_mouse;
use meshtext::{MeshGenerator, MeshText, TextSection};

pub fn init_bevy_app(text_receiver: BevyEventReceiver<TextEvent>) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy_canvas".into()),
                    resolution: WindowResolution::new(RENDER_WIDTH, RENDER_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
        // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .import_event_from_leptos(text_receiver)
    .add_systems(Startup, (setup_scene,))
    .add_systems(Update, (camera_from_mouse, update_text));

    app
}

fn update_text(mut commands: Commands, mut event_reader: EventReader<TextEvent>) {
    for event in event_reader.read() {
        leptos::logging::log!("Received text event: {}", event.text);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

    let font_data = include_bytes!("../assets/fonts/JetBrainsMono-ExtraBold.ttf");
    let mut generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(1.0, 1.0, 0.2)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(&"HELLO WORLD!".to_string(), false, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0.0, 0.0]; positions.len()];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();

    // text
    commands
        // use this bundle to change the rotation pivot to the center
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            // transform mesh so that it is in the center
            Transform::from_translation(Vec3::new(-text_mesh.bbox.size().x * 0.5, 0.0, 0.0)),
        ));

    // Spawn the shapes. The meshes will be pickable by default.
    // for (i, shape) in shapes.into_iter().enumerate() {
    //     commands
    //         .spawn((
    //             Mesh3d(shape),
    //             MeshMaterial3d(white_matl.clone()),
    //             Transform::from_xyz(
    //                 -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
    //                 2.0,
    //                 Z_EXTENT / 2.,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             Shape,
    //         ))
    //         .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    //         .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    //         .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
    //         .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
    //         .observe(rotate_on_drag);
    // }

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(ground_matl.clone()),
        PickingBehavior::IGNORE, // Disable picking for the ground plane.
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
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}
