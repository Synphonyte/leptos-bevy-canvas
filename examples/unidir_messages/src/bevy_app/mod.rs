mod components;
mod resources;
mod setup;
mod systems;

use crate::bevy_app::resources::*;
use crate::bevy_app::setup::setup_scene;
use crate::bevy_app::systems::*;
use crate::messages::{ClickMessage, TextMessage};
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use leptos_bevy_canvas::prelude::*;

pub fn init_bevy_app(
    text_receiver: BevyMessageReceiver<TextMessage>,
    click_sender: BevyMessageSender<ClickMessage>,
) -> App {
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
        // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ))
    .init_resource::<CurrentText>()
    .init_resource::<SelectedGlyph>()
    .import_message_from_leptos(text_receiver)
    .export_message_to_leptos(click_sender)
    .add_systems(Startup, (setup_scene,))
    .add_systems(Update, (update_text,));

    app
}
