use crate::bevy_app::init_bevy_app;
use crate::events::{ClickEvent, TextEvent};
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::prelude::Name;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::use_debounce_fn;

#[derive(Copy, Clone)]
pub enum EventDirection {
    None,
    LeptosToBevy,
    BevyToLeptos,
}

#[component]
pub fn App() -> impl IntoView {
    let (selected, selected_query_duplex) = BevyQuery::<&mut Name>::signal();

    view! {
        <div class="flex w-full mx-auto max-w-[1400px] p-5 items-center">
            <Frame class="border-red-500 bg-red-500/5 flex-1">
                <h2 class="text-xl font-bold text-red-500 relative top-[-10px]">Bevy</h2>
                <div
                    class="aspect-[6/5] rounded-lg overflow-hidden"
                    style:max-width=format!("{}px", RENDER_WIDTH)
                    style:max-height=format!("{}px", RENDER_HEIGHT)
                >
                    <BevyCanvas
                        init=move || { init_bevy_app(selected_query_duplex) }
                        {..}
                        width=RENDER_WIDTH
                        height=RENDER_HEIGHT
                    />
                </div>
            </Frame>

            <Frame class="border-blue-500 bg-blue-500/5 max-w-[200px]">
                <h2 class="text-xl font-bold text-blue-500 relative top-[-10px]">Leptos</h2>
            </Frame>
        </div>
    }
}

#[component]
pub fn TextDisplay(
    text: ReadSignal<String>,
    click_event_receiver: LeptosEventReceiver<ClickEvent>,
) -> impl IntoView {
    view! {
        <div class="mt-3 text-sm font-medium text-white">
            Preview
        </div>
        <div class="mt-2 border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 text-white">
            <For
                each=move || { text.get().chars().enumerate().collect::<Vec<_>>() }
                key=|(i, _)| *i
                children=move |(i, c)| {
                    let class = move || {
                        let class = if let Some(event) = click_event_receiver.get() {
                            if event.char_index == i { "top-[-5px]" } else { "top-0" }
                        } else {
                            "top-0"
                        };

                        format!(
                            "relative inline-block transition-all duration-200 ease-out {class}",
                        )
                    };

                    view! { <span class=class>{c}</span> }
                }
            />
        </div>
    }
}


#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}
