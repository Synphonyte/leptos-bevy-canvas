mod leptos_app;
mod bevy_app;
mod events;

use crate::leptos_app::App;
use leptos::prelude::mount_to_body;

pub const RENDER_WIDTH: f32 = 600.0;
pub const RENDER_HEIGHT: f32 = 500.0;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
