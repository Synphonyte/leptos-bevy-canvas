use crate::bevy_app::components::*;
use bevy::prelude::*;

pub fn apply_rotation(mut query: Query<(&mut Transform, &RotationSpeed)>) -> Result<(), BevyError> {
    for (mut transform, rotation_speed) in query.iter_mut() {
        transform.rotate_y(**rotation_speed);
    }
    Ok(())
}

pub fn apply_color(
    query: Query<(&ObjectColor, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), BevyError> {
    for (color, material) in &query {
        if let Some(material) = materials.get_mut(material.0.id()) {
            material.base_color = **color;
        }
    }
    Ok(())
}

pub fn selected_outline(
    mut query: Query<(&ChildOf, &mut Visibility)>,
    selected_query: Query<&Selected>,
) -> Result<(), BevyError> {
    for (parent, mut visibility) in query.iter_mut() {
        if selected_query.contains(parent.parent()) {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    Ok(())
}
