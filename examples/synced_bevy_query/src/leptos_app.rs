use crate::bevy_app::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::color::{palettes::tailwind::*, Srgba};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (selected, selected_query_duplex) =
        single_query_signal::<(RotationSpeed, ObjectColor, Selected), ()>();

    Effect::new(move || {
        leptos::logging::log!("changed: {:?}", selected.get());
    });

    let inputs_disabled = Signal::derive(move || selected.read().is_none());

    view! {
        <div class="flex gap-5 items-center p-5 mx-auto w-full max-w-[1400px]">
            <Frame
                class="border-red-500 flex-4 bg-red-500/5"
                {..}
                style=format!("max-width: calc(2.5rem + {RENDER_WIDTH}px);")
            >
                <div class="float-right">Click on a cube to select</div>

                <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
                <div
                    class="overflow-hidden rounded-lg aspect-[8/5]"
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

            <Frame class="flex-1 border-blue-500 bg-blue-500/5 max-w-[370px]">
                <h2 class="relative text-xl font-bold text-blue-500 top-[-10px]">Leptos</h2>

                <div class=move || {
                    if inputs_disabled.get() { "pointer-events-none opacity-50" } else { "" }
                }>
                    <label class="block mt-2 mb-2">Color</label>
                    <TailwindColorSelector
                        value=Signal::derive(move || {
                            selected.read().as_ref().map(|(_, color, _)| color.clone())
                        })
                        on_input=move |new_color| {
                            selected.write().as_mut().map(|(_, color, _)| { *color = new_color });
                        }
                    />

                    <label class="block mt-6 mb-2">Rotation speed</label>
                    <input
                        class="block w-full"
                        type="range"
                        min="-0.1"
                        max="0.1"
                        step="0.01"
                        prop:value=move || {
                            selected
                                .read()
                                .as_ref()
                                .map(|(rotation_speed, _, _)| rotation_speed.to_string())
                                .unwrap_or_default()
                        }
                        on:input=move |ev| {
                            selected
                                .write()
                                .as_mut()
                                .map(|(rotation_speed, _, _)| {
                                    if let Ok(value) = event_target_value(&ev).parse() {
                                        **rotation_speed = value;
                                    }
                                });
                        }
                    />
                </div>
            </Frame>
        </div>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}

#[component]
pub fn TailwindColorSelector(
    value: Signal<Option<ObjectColor>>,
    #[prop(into)] on_input: Callback<(ObjectColor,)>,
) -> impl IntoView {
    const COLORS: [Srgba; 16] = [
        YELLOW_500,
        AMBER_500,
        ORANGE_500,
        RED_500,
        VIOLET_500,
        PURPLE_500,
        FUCHSIA_500,
        PINK_500,
        INDIGO_500,
        BLUE_500,
        SKY_500,
        CYAN_500,
        LIME_500,
        GREEN_500,
        EMERALD_500,
        TEAL_500,
    ];

    view! {
        <div class="grid grid-cols-4 gap-2">
            <For each=move || COLORS key=|c| c.to_hex() let:color>
                <div
                    class=move || {
                        format!(
                            "h-8 rounded {}",
                            if value.get() == Some(ObjectColor::new(color.into())) {
                                "border-2 border-white"
                            } else {
                                ""
                            },
                        )
                    }
                    style:background-color=color.to_hex()
                    on:click=move |_| {
                        on_input.run((ObjectColor::new(color.into()),));
                    }
                ></div>
            </For>
        </div>
    }
}
