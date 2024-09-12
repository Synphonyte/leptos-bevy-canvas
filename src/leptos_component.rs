use bevy::prelude::*;
use leptos::prelude::*;

#[component]
pub fn BevyCanvas(
    init: impl FnOnce() -> App + 'static,
    #[prop(into, default = "bevy_canvas".to_string())] canvas_id: String,
) -> impl IntoView {
    request_animation_frame(move || {
        let mut app = init();
        app.run();
    });

    view! { <canvas id=canvas_id></canvas> }
}
