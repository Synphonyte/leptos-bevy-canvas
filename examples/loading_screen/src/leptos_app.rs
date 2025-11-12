use crate::bevy_app::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (state, bevy_state) = signal_synced(AppState::default());

    Effect::new(move || {
        leptos::logging::log!("AppState changed to {:?}", state.get());
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
                    <LeptosOverlay state />

                    <Show
                        when=move || state.get() != AppState::None
                        fallback=move || view! { <StartBevyButton state /> }
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

#[component]
pub fn StartBevyButton(state: RwSignalSynced<AppState>) -> impl IntoView {
    view! {
        <div class="flex items-center">
            <button
                type="button"
                class="absolute py-2 px-3.5 text-sm font-semibold text-white bg-blue-500 rounded-full ring-1 ring-inset ring-gray-300 shadow-sm hover:bg-red-600 -translate-x-1/2 -translate-y-1/2"
                on:click=move |_| {
                    state.set(AppState::Start);
                }
            >
                Start Bevy
            </button>
        </div>
    }
}

#[component]
pub fn LeptosOverlay(state: RwSignalSynced<AppState>) -> impl IntoView {
    let is_loading =
        Signal::derive(move || matches!(state.get(), AppState::Start | AppState::Loading));

    let loading_class = Signal::derive(move || {
        format!(
        "absolute top-1/2 left-1/2 w-1/2 text-lg text-center text-white -translate-x-1/2 -translate-y-1/2 {}",
        if !is_loading.get() { "hidden" } else { "" }
    )
    });

    view! {
        <AnimatedShow
            when=Signal::derive(move || state.get() != AppState::Ready)
            show_class="fade-in-1000"
            hide_class="fade-out-1000"
            hide_delay=std::time::Duration::from_millis(1000)
        >
            <div class="absolute top-0 left-0 w-full h-full rounded-lg border-2 border-blue-500 border-solid bg-blue-500/5 backdrop-blur-sm">
                <h2 class="absolute text-xl font-bold text-blue-500 top-[10px] left-[10px]">
                    Leptos Overlay
                </h2>

                <div class=loading_class>
                    <div class="relative w-full text-blue-500 rounded-md">
                        <svg
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                            class="scale-[0.1] fill-blue-400"
                        >
                            <defs>
                                <filter id="spinner-gF01">
                                    <feGaussianBlur
                                        in="SourceGraphic"
                                        stdDeviation="1"
                                        result="y"
                                    />
                                    <feColorMatrix
                                        in="y"
                                        mode="matrix"
                                        values="1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 18 -7"
                                        result="z"
                                    />
                                    <feBlend in="SourceGraphic" in2="z" />
                                </filter>
                            </defs>
                            <g filter="url(#spinner-gF01)">
                                <circle cx="5" cy="12" r="4">
                                    <animate
                                        attributeName="cx"
                                        calcMode="spline"
                                        dur="2s"
                                        values="5;8;5"
                                        keySplines=".36,.62,.43,.99;.79,0,.58,.57"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                                <circle cx="19" cy="12" r="4">
                                    <animate
                                        attributeName="cx"
                                        calcMode="spline"
                                        dur="2s"
                                        values="19;16;19"
                                        keySplines=".36,.62,.43,.99;.79,0,.58,.57"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                                <animateTransform
                                    attributeName="transform"
                                    type="rotate"
                                    dur="0.75s"
                                    values="0 12 12;360 12 12"
                                    repeatCount="indefinite"
                                />
                            </g>
                        </svg>
                    </div>
                </div>
            </div>
        </AnimatedShow>
    }
}
