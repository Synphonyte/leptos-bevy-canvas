mod components;
mod resources;
mod setup;
mod states;
mod systems;

use crate::bevy_app::resources::AssetsLoading;
use crate::bevy_app::setup::setup_scene;
pub use crate::bevy_app::states::AppState;
use crate::bevy_app::systems::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::asset::io::web::WebAssetPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use leptos_bevy_canvas::prelude::*;

pub fn init_bevy_app(bevy_loading_state: BevyMessageDuplex<AppState>) -> App {
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
            })
            .set(WebAssetPlugin {
                silence_startup_warning: true,
            }),
        // bevy_inspector_egui::bevy_egui::EguiPlugin {
        //     enable_multipass_for_primary_context: true,
        // },
        // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .add_plugins(PanOrbitCameraPlugin)
    // this also inits the state with Bevy
    .sync_leptos_signal_with_state(bevy_loading_state)
    .init_resource::<AssetsLoading>()
    .add_systems(Startup, (setup_scene, toogle_between_assets))
    .add_systems(
        Update,
        (
            track_assets_loading,
            toogle_between_assets
                .run_if(in_state(AppState::Ready))
                .run_if(|keyboard: Res<ButtonInput<KeyCode>>| {
                    keyboard.just_pressed(KeyCode::Space)
                }),
        ),
    );
    app
}
