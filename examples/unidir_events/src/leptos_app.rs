use crate::bevy_app::init_bevy_app;
use crate::events::TextEvent;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (text_event_sender, text_receiver) = event_l2b::<TextEvent>();

    let canvas_width = RENDER_WIDTH * 3.0;

    view! {
        <div
            class="relative z-10 w-full aspect-[9/5] mx-auto"
            style:max-width=format!("{}px", canvas_width)
            style:max-height=format!("{}px", RENDER_HEIGHT)
        >
            <BevyCanvas
                init=move || { init_bevy_app(text_receiver) }
                {..}
                width=canvas_width
                height=RENDER_HEIGHT
            />
        </div>

        <div class="p-5">
            <TextInput event_sender=text_event_sender />
        </div>
    }
}

#[component]
pub fn TextInput(event_sender: LeptosEventSender<TextEvent>) -> impl IntoView {
    let on_input = move |evt| {
        event_sender
            .send(TextEvent {
                text: event_target_value(&evt),
            })
            .ok();
    };

    view! {
        <div>
            <label for="some-text" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                The text
            </label>
            <input
                id="some-text"
                type="text"
                 placeholder="Enter something"
                on:input=on_input
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            />
        </div>
    }
}
