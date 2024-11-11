mod bevy;
mod leptos_app;

use crate::leptos_app::App;
use leptos::prelude::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
