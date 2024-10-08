use crossbeam_channel::Receiver;
use leptos::prelude::*;
use leptos_use::use_raf_fn;

pub(crate) fn init_rw_signal_from_receiver<E>(rx: &Receiver<E>) -> RwSignal<Option<E>>
where
    E: Send + Sync + 'static,
{
    let signal = RwSignal::new(None);

    use_raf_fn({
        let rx = rx.clone();

        move |_| {
            for event in rx.try_iter() {
                signal.set(Some(event));
            }
        }
    });

    signal
}
