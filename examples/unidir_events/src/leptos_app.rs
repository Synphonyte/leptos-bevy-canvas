use crate::bevy_app::init_bevy_app;
use crate::events::{ClickEvent, TextEvent};
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use leptos::prelude::Set;
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
    let (text_event_sender, text_receiver) = event_l2b::<TextEvent>();
    let (click_event_receiver, click_event_sender) = event_b2l::<ClickEvent>();

    let (text, set_text) = signal(String::new());
    let (event_str, set_event_str) = signal(String::new());
    let (event_direction, set_event_direction) = signal(EventDirection::None);

    let on_input = move |text: String| {
        set_text.set(text.clone());

        let text_event = TextEvent { text };

        set_event_str.set(format!("{:#?}", text_event));
        set_event_direction.set(EventDirection::LeptosToBevy);

        text_event_sender.send(text_event).ok();
    };

    Effect::new(move || {
        if let Some(event) = click_event_receiver.get() {
            set_event_str.set(format!("{:#?}", event));
            set_event_direction.set(EventDirection::BevyToLeptos);
        }
    });

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
                        init=move || { init_bevy_app(text_receiver, click_event_sender) }
                        {..}
                        width=RENDER_WIDTH
                        height=RENDER_HEIGHT
                    />
                </div>
            </Frame>

            <EventDisplay event_str event_direction />

            <Frame class="border-blue-500 bg-blue-500/5 max-w-[200px]">
                <h2 class="text-xl font-bold text-blue-500 relative top-[-10px]">Leptos</h2>
                <TextInput on_input=on_input />
                <TextDisplay text click_event_receiver />
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
pub fn EventDisplay(
    event_str: ReadSignal<String>,
    event_direction: ReadSignal<EventDirection>,
) -> impl IntoView {
    let (event_display_class, set_event_display_class) = signal("opacity-0".to_string());

    let reset_event_display_class = move || {
        set_event_display_class
            .set("opacity-30 transition-opacity duration-1000 ease-in".to_string())
    };
    let debounced_reset_event_display_class = use_debounce_fn(reset_event_display_class, 500.0);
    let activate_event_display = move || {
        set_event_display_class.set("opacity-100".to_string());
        debounced_reset_event_display_class();
    };

    Effect::watch(
        move || event_str.track(),
        move |_, _, _| {
            activate_event_display();
        },
        false,
    );

    view! {
        <div class="flex-1 px-5 relative">
            <EventDirectionIndicator event_direction />
            <pre class=move || {
                format!(
                    "overflow-x-auto bg-gray-700 border border-gray-600 rounded p-3 absolute top-[30px] max-w-[80%] left-1/2 -translate-x-1/2 {}",
                    event_display_class.get(),
                )
            }>
                <code>{event_str}</code>
            </pre>
        </div>
    }
}

#[component]
pub fn EventDirectionIndicator(event_direction: ReadSignal<EventDirection>) -> impl IntoView {
    let color = Signal::derive(move || match event_direction.get() {
        EventDirection::LeptosToBevy => "rgb(59, 130, 246)",
        EventDirection::BevyToLeptos => "rgb(239, 68, 68)",
        EventDirection::None => "transparent",
    });

    let transform = Signal::derive(move || match event_direction.get() {
        EventDirection::LeptosToBevy => "scale(1, 1)",
        EventDirection::BevyToLeptos => "scale(-1, 1)",
        EventDirection::None => "scale(1, 1)",
    });

    // svg arrow
    view! {
        <svg width="100%" height="20">
            <g style:transform=transform style:transform-origin="50% 50%">
                <path d="M20 0 L0 10 L20 20 z" fill=color />
                <line x1="20" y1="10" x2="100%" y2="10" stroke=color stroke-width="2" />
            </g>
        </svg>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}

#[component]
pub fn TextInput(#[prop(into)] on_input: Callback<(String,)>) -> impl IntoView {
    let (value, set_value) = signal(String::new());

    let on_input = move |evt| {
        let text = event_target_value(&evt).replace(" ", "");
        set_value.set(text.clone());
        on_input.run((text,));
    };

    view! {
        <div>
            <label for="some-text" class="block mb-2 text-sm font-medium text-white">
                Input
            </label>
            <input
                id="some-text"
                type="text"
                placeholder="Enter something"
                on:input=on_input
                prop:value=value
                class="border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
            />
        </div>
    }
}
