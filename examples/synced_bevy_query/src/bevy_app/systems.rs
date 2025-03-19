use crate::bevy_app::components::*;
use bevy::prelude::*;

pub fn apply_rotation(mut query: Query<(&mut Transform, &RotationSpeed)>) {
    for (mut transform, rotation_speed) in query.iter_mut() {
        transform.rotate_y(**rotation_speed);
    }
}

pub fn apply_color(
    query: Query<(&ObjectColor, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (color, material) in &query {
        if let Some(material) = materials.get_mut(material.0.id()) {
            material.base_color = **color;
        }
    }
}

pub fn selected_outline(
    mut query: Query<(&Parent, &mut Visibility)>,
    selected_query: Query<&Selected>,
) {
    for (parent, mut visibility) in query.iter_mut() {
        if selected_query.contains(parent.get()) {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
