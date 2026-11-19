use bevy::prelude::*;
use leptos::prelude::*;
use leptos_use_raf_fn::use_raf_fn;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

type StartAppFn = Rc<RefCell<Option<Box<dyn FnOnce()>>>>;
type PauseFn = Rc<RefCell<Option<Box<dyn Fn()>>>>;

use crate::{
    messages::{message_l2b, LeptosChannelMessageSender},
    plugin::{LeptosBevyCanvasCleanup, LeptosBevyCanvasPlugin},
    prelude::LeptosBevyApp,
};

/// Set when a BevyCanvas is destroyed. Cleared by the Bevy cleanup system
/// after it sends `AppExit`, allowing the next mount's polling loop to proceed.
pub(crate) static AWAITING_CLEANUP: AtomicBool = AtomicBool::new(false);

/// Embeds a Bevy app in a Leptos component. It will add an HTML canvas element and start
/// running the Bevy app inside it.
#[component]
pub fn BevyCanvas(
    /// This function is be called to initialize and return the Bevy app.
    init: impl FnOnce() -> App + 'static,
    /// Optional canvas id. Defaults to `bevy_canvas`.
    #[prop(into, default = "bevy_canvas".to_string())]
    canvas_id: String,
) -> impl IntoView {
    let (shutdown_canvas, set_shutdown_canvas) = message_l2b::<LeptosBevyCanvasCleanup>();

    let start_app: StartAppFn = Rc::new(RefCell::new(Some(Box::new(move || {
        let mut app = init();
        app.add_plugins(LeptosBevyCanvasPlugin)
            .import_message_from_leptos(set_shutdown_canvas);
        app.run();
    }))));

    let pause_polling: PauseFn = Rc::new(RefCell::new(None));
    let app_started = Arc::new(AtomicBool::new(false));

    // Poll each frame until winit's event-loop cleanup is done, then start.
    let raf = use_raf_fn({
        let start_app = start_app.clone();
        let pause_polling = pause_polling.clone();
        let app_started = app_started.clone();
        move |_| {
            if !AWAITING_CLEANUP.load(Ordering::Relaxed) {
                if let Some(start) = start_app.borrow_mut().take() {
                    app_started.store(true, Ordering::Relaxed);
                    request_animation_frame(start);
                    // Stop polling now that the app is running.
                    if let Some(pause) = pause_polling.borrow_mut().take() {
                        pause();
                    }
                }
            }
        }
    });

    // Make the pause handle available to the polling callback.
    *pause_polling.borrow_mut() = Some(Box::new(raf.pause.clone()));

    on_cleanup(move || {
        (raf.pause)();
        if !app_started.load(Ordering::Relaxed) {
            // App never started, no CleanupFlag exists to clear this.
            return;
        }
        AWAITING_CLEANUP.store(true, Ordering::Relaxed);
        shutdown_canvas
            .send(LeptosBevyCanvasCleanup)
            .expect("couldn't send cleanup to bevy app");
    });

    view! { <canvas id=canvas_id></canvas> }
}
