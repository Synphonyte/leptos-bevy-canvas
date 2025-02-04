mod components;
mod resources;
mod setup;
mod systems;

use crate::bevy_app::setup::setup_scene;
use crate::bevy_app::systems::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rand::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_bevy_canvas_examples::camera_from_mouse;

pub fn init_bevy_app(selected_query_duplex: BevyQueryDuplex<(Name,), With<Transform>>) -> App {
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
        MeshPickingPlugin,
        EntropyPlugin::<WyRand>::default(),
        bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .sync_leptos_signal_with_query(selected_query_duplex)
    .add_systems(Startup, (setup_scene,))
    .add_systems(
        Update,
        (
            camera_from_mouse,
            change_name.run_if(input_pressed(KeyCode::Enter)),
        ),
    );

    app
}
