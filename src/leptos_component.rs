use bevy::prelude::*;
use leptos::prelude::*;

use crate::{
    events::{event_l2b, LeptosChannelEventSender},
    plugin::{LeptosBevyCanvasCleanup, LeptosBevyCanvasPlugin},
    prelude::LeptosBevyApp,
};

/// Embeds a Bevy app in a Leptos component. It will add an HTML canvas element and start
/// running the Bevy app inside it.
#[component]
pub fn BevyCanvas(
    /// This function is be called to initialize and return the Bevy app.
    init: impl FnOnce() -> App + 'static,
    /// Optional canvas id. Defaults to `bevy_canvas`.
    #[prop(into, default = "bevy_canvas".to_string())]
    canvas_id: String,
) -> impl IntoView {
    let (shutdown_canvas, set_shutdown_canvas) = event_l2b::<LeptosBevyCanvasCleanup>();

    request_animation_frame(move || {
        let mut app = init();
        app.add_plugins(LeptosBevyCanvasPlugin)
            .import_event_from_leptos(set_shutdown_canvas);
        app.run();
    });

    on_cleanup(move || {
        shutdown_canvas
            .send(LeptosBevyCanvasCleanup)
            .expect("couldn't send cleanup to bevy app");
    });

    view! { <canvas id=canvas_id></canvas> }
}
