use crate::bevy_app::components::CharIndex;
use crate::bevy_app::resources::*;
use crate::events::{ClickEvent, TextEvent};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::color::palettes::tailwind::GREEN_100;
use bevy::color::Color;
use bevy::math::{Mat4, Vec3};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use meshtext::{Face, Glyph, MeshGenerator, MeshText};

const LETTER_Y_ANGLE_STEP: f32 = 0.08;

pub fn update_text(
    mut commands: Commands,
    mut event_reader: EventReader<TextEvent>,
    mut current_text: ResMut<CurrentText>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected_glyph: ResMut<SelectedGlyph>,
) {
    for event in event_reader.read() {
        let transform_step: Transform =
            Transform::from_rotation(Quat::from_rotation_y(LETTER_Y_ANGLE_STEP));

        let font_data = include_bytes!("../../assets/fonts/FiraMono-Regular.ttf");
        let mut generator = MeshGenerator::new(font_data);

        let mut transform = Transform::from_xyz(0.0, 0.0, 6.0);
        let mut new_glyph_entites = Vec::new();

        for (i, ((existing_glyph, event_glyph), existing_glyph_entity)) in current_text
            .text
            .chars()
            .zip(event.text.chars())
            .zip(current_text.glyph_entities.iter().cloned())
            .enumerate()
        {
            if existing_glyph != event_glyph {
                let new_glyph_entity = spawn_letter(
                    event_glyph,
                    i,
                    transform,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut generator,
                );

                new_glyph_entites.push(new_glyph_entity);
                commands.entity(existing_glyph_entity).despawn();
            } else {
                new_glyph_entites.push(existing_glyph_entity);
            }

            transform = transform_step * transform;
        }

        let diff = current_text.text.len() as i32 - event.text.len() as i32;

        if diff > 0 {
            for entity in current_text.glyph_entities.iter().skip(event.text.len()) {
                commands.entity(*entity).despawn();
            }
        } else if diff < 0 {
            let mut i = current_text.text.len();
            for glyph in event.text.chars().skip(i) {
                let glyph_transform = if let SelectedGlyph::Some { index, .. } = *selected_glyph {
                    if index == i {
                        let mut t = transform.clone();
                        t.translation.y = 0.5;
                        t
                    } else {
                        transform
                    }
                } else {
                    transform
                };

                let new_glyph_entity = spawn_letter(
                    glyph,
                    i,
                    glyph_transform,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut generator,
                );
                new_glyph_entites.push(new_glyph_entity);

                if let SelectedGlyph::Some { index, entity } = &mut *selected_glyph {
                    if *index == i {
                        *entity = new_glyph_entity;
                    }
                }

                transform = transform_step * transform;
                i += 1;
            }
        }

        current_text.glyph_entities = new_glyph_entites;
        current_text.text = event.text.clone();
    }
}

fn spawn_letter(
    glyph: char,
    glyph_index: usize,
    glyph_transform: Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    text_generator: &mut MeshGenerator<Face>,
) -> Entity {
    let transform = Mat4::from_scale(Vec3::new(1.0, 1.0, 0.2)).to_cols_array();
    let text_mesh: MeshText = text_generator
        .generate_glyph(glyph, false, Some(&transform))
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
    let glyph_width = text_mesh.bbox.size().x;
    let entity = commands
        // use this bundle to change the rotation pivot to the center
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(GREEN_100),
                perceptual_roughness: 0.1,
                reflectance: 1.0,
                ..default()
            })),
            // transform mesh so that it is in the center
            glyph_transform * Transform::from_translation(Vec3::new(-glyph_width * 0.5, 0.0, 0.0)),
            CharIndex::new(glyph_index),
        ))
        .observe(on_char_click)
        .id();

    entity
}

fn on_char_click(
    trigger: Trigger<Pointer<Down>>,
    index_query: Query<&CharIndex>,
    mut transform_query: Query<&mut Transform>,
    mut event_writer: EventWriter<ClickEvent>,
    mut selected_glyph: ResMut<SelectedGlyph>,
) {
    let entity = trigger.entity();

    if let Ok(index) = index_query.get(entity) {
        let index = **index;

        if let SelectedGlyph::Some { entity, .. } = *selected_glyph {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation.y = 0.0;
            }
        }

        *selected_glyph = SelectedGlyph::Some { entity, index };

        if let Ok(mut transform) = transform_query.get_mut(entity) {
            transform.translation.y = 0.5;
        }

        event_writer.send(ClickEvent { char_index: index });
    }
}
