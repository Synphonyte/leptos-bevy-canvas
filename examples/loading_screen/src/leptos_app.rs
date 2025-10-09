use crate::bevy_app::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (state, bevy_state) = signal_synced(AppState::default());

    Effect::new(move || {
        leptos::logging::log!("changed: {:?}", state.get());
    });

    view! {
        <div
            class="block gap-5 items-center p-5 mx-auto w-full max-w-[1400px]"
            style:max-width=format!("{}px", RENDER_WIDTH)
        >
            <div class="p-5 rounded-lg border-2 border-red-500 border-solid flex-4 bg-red-500/5">
                <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
                <div
                    class="flex overflow-hidden relative justify-center rounded-lg aspect-[8/5]"
                    style:max-width=format!("{}px", RENDER_WIDTH)
                    style:max-height=format!("{}px", RENDER_HEIGHT)
                >
                    <AnimatedShow
                        when=Signal::derive(move || {
                            matches!(state.get(), AppState::Start | AppState::Loading)
                        })
                        show_class="fade-in-1000"
                        hide_class="fade-out-1000"
                        hide_delay=std::time::Duration::from_millis(1000)
                    >
                        <div class="absolute top-0 left-0 w-full h-full rounded-lg border-2 border-blue-500 border-solid bg-blue-500/5 backdrop-blur-sm">
                            <h2 class="absolute text-xl font-bold text-blue-500 top-[10px] left-[10px]">
                                Leptos Overlay
                            </h2>
                            <div class="absolute top-1/2 left-1/2 w-1/2 text-lg text-center text-white -translate-x-1/2 -translate-y-1/2">
                                <div class="relative w-full text-blue-500 rounded-md">
                                    <svg
                                        version="1.1"
                                        id="L7"
                                        xmlns="http://www.w3.org/2000/svg"
                                        xmlns:xlink="http://www.w3.org/1999/xlink"
                                        x="0px"
                                        y="0px"
                                        viewBox="0 0 100 100"
                                        enable-background="new 0 0 100 100"
                                        xml:space="preserve"
                                        class="transform-gpu stroke-white scale-[0.3]"
                                    >
                                        <path
                                            fill="currentColor"
                                            d="M31.6,3.5C5.9,13.6-6.6,42.7,3.5,68.4c10.1,25.7,39.2,38.3,64.9,28.1l-3.1-7.9c-21.3,8.4-45.4-2-53.8-23.3
                                            c-8.4-21.3,2-45.4,23.3-53.8L31.6,3.5z"
                                        >
                                            <animateTransform
                                                attributeName="transform"
                                                attributeType="XML"
                                                type="rotate"
                                                dur="2s"
                                                from="0 50 50"
                                                to="360 50 50"
                                                repeatCount="indefinite"
                                            />
                                        </path>
                                        <path
                                            fill="currentColor"
                                            d="M42.3,39.6c5.7-4.3,13.9-3.1,18.1,2.7c4.3,5.7,3.1,13.9-2.7,18.1l4.1,5.5c8.8-6.5,10.6-19,4.1-27.7
                                            c-6.5-8.8-19-10.6-27.7-4.1L42.3,39.6z"
                                        >
                                            <animateTransform
                                                attributeName="transform"
                                                attributeType="XML"
                                                type="rotate"
                                                dur="1s"
                                                from="0 50 50"
                                                to="-360 50 50"
                                                repeatCount="indefinite"
                                            />
                                        </path>
                                        <path
                                            fill="currentColor"
                                            d="M82,35.7C74.1,18,53.4,10.1,35.7,18S10.1,46.6,18,64.3l7.6-3.4c-6-13.5,0-29.3,13.5-35.3s29.3,0,35.3,13.5
                                            L82,35.7z"
                                        >
                                            <animateTransform
                                                attributeName="transform"
                                                attributeType="XML"
                                                type="rotate"
                                                dur="2s"
                                                from="0 50 50"
                                                to="360 50 50"
                                                repeatCount="indefinite"
                                            />
                                        </path>
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </AnimatedShow>
                    <Show
                        when=move || state.get() != AppState::None
                        fallback=move || {
                            view! {
                                <div class="flex items-center">
                                    <button
                                        type="button"
                                        class="py-2 px-3.5 text-sm font-semibold text-white bg-blue-500 rounded-full ring-1 ring-inset ring-gray-300 shadow-sm hover:bg-red-600"
                                        on:click=move |_| {
                                            state.set(AppState::Start);
                                        }
                                    >
                                        Start Bevy
                                    </button>
                                </div>
                            }
                        }
                    >
                        <BevyCanvas
                            init={
                                let bevy_state = bevy_state.clone();
                                move || { init_bevy_app(bevy_state) }
                            }
                            {..}
                            width=RENDER_WIDTH
                            height=RENDER_HEIGHT
                        />
                    </Show>
                </div>
            </div>
            <Show when=move || state.get() == AppState::Ready>
                <h2 class="text-sm font-bold text-center text-blue-500">
                    Press space to select the next model
                </h2>
            </Show>
        </div>
    }
}
