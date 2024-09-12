use bevy::prelude::*;
use leptos::prelude::*;

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
    request_animation_frame(move || {
        let mut app = init();
        app.run();
    });

    view! { <canvas id=canvas_id></canvas> }
}
