use bevy::prelude::*;

/// Amount of y-rotation applied every frame.
#[derive(Component, Deref, DerefMut, Copy, Clone, Debug)]
pub struct RotationSpeed(f32);

/// Marker component for selected entities. Only the one selected entity has this component.
#[derive(Component, Copy, Clone, Debug)]
pub struct Selected;

/// Color of the object. Will be synchronized to the material.
#[derive(Component, Deref, DerefMut, Clone, Debug, PartialEq)]
pub struct ObjectColor(Color);

impl RotationSpeed {
    pub fn new(speed: f32) -> Self {
        RotationSpeed(speed)
    }
}

impl ObjectColor {
    pub fn new(color: Color) -> Self {
        ObjectColor(color)
    }
}
