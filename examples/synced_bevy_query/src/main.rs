mod bevy_app;
mod leptos_app;

use crate::leptos_app::App;
use leptos::prelude::mount_to_body;

pub const RENDER_WIDTH: u32 = 996;
pub const RENDER_HEIGHT: u32 = 622;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
