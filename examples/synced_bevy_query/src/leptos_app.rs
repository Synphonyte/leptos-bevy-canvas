use crate::bevy_app::init_bevy_app;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::prelude::{Name, Transform, With};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (selected, selected_query_duplex) = single_query_signal::<(Name,), With<Transform>>();

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

                <input
                    type="text"
                    prop:value=move || selected.read().as_ref().map(|(name,)| name.to_string()).unwrap_or_default()
                    on:input=move |ev| {
                        selected.write().as_mut().map(|(name,)| name.set(event_target_value(&ev)));
                    }
                />
            </Frame>
        </div>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}
