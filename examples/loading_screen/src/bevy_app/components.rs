use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Model {
    /// ![Source](https://science.nasa.gov/3d-resources/gateway-lunar-space-station/)
    GatewayCore,
    /// ![Source](https://sketchfab.com/3d-models/destructible-robot-highpoly-0f9c08f0d8d94f3a9938d4d8bf423f46)
    Robot,
}

impl Model {
    pub fn src(&self) -> String {
        match self {
            Model::GatewayCore => "Gateway Core.glb",
            Model::Robot => "Robot.glb",
        }
        .to_string()
    }

    pub fn initial_transform(&self) -> Transform {
        match self {
            Model::GatewayCore => Transform::from_translation(Vec3::new(0.0, 0.2, 0.0))
                .with_scale(Vec3::new(0.01, 0.01, 0.01)),
            Model::Robot => Transform::from_translation(Vec3::new(0.0, 0.1, 0.0))
                .with_scale(Vec3::new(0.001, 0.001, 0.001)),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Model::GatewayCore => Model::Robot,
            Model::Robot => Model::GatewayCore,
        }
    }
}
