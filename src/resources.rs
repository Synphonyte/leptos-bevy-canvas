use crate::events::BevyEventDuplex;
use crossbeam_channel::Sender;
use leptos::prelude::guards::{Plain, ReadGuard};
use leptos::prelude::*;
use leptos_use::use_raf_fn;
use std::ops::DerefMut;
use std::panic::Location;

/// This is basically identical to a Leptos `RwSignal` but is automatically synced with a Bevy
/// `Resource`.
pub struct RwSignalResource<T> {
    rw_signal: RwSignal<T>,
    tx: StoredValue<Sender<T>>,
}

impl<T> Clone for RwSignalResource<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RwSignalResource<T> {}

impl<T> DefinedAt for RwSignalResource<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.rw_signal.defined_at()
    }
}

impl<T> IsDisposed for RwSignalResource<T>
where
    T: 'static,
{
    fn is_disposed(&self) -> bool {
        self.rw_signal.is_disposed()
    }
}

impl<T> ReadUntracked for RwSignalResource<T>
where
    T: 'static,
    RwSignal<T>: ReadUntracked<Value = ReadGuard<T, Plain<T>>>,
{
    type Value = ReadGuard<T, Plain<T>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.rw_signal.try_read_untracked()
    }
}

impl<T> Track for RwSignalResource<T>
where
    RwSignal<T>: Track,
{
    fn track(&self) {
        self.rw_signal.track();
    }
}

impl<T> Notify for RwSignalResource<T>
where
    RwSignal<T>: Notify,
{
    fn notify(&self) {
        self.rw_signal.notify();
    }
}

impl<T> Write for RwSignalResource<T>
where
    T: Send + Clone + 'static,
    RwSignal<T>: Write<Value = T> + GetUntracked<Value = T>,
{
    type Value = T;

    fn try_write(&self) -> Option<impl UntrackableGuard<Target = Self::Value>> {
        let inner_guard = self.rw_signal.try_write()?;

        request_animation_frame({
            let rw_signal = self.rw_signal;
            let tx = self.tx;

            move || {
                tx.with_value(|tx| {
                    tx.send(rw_signal.get_untracked())
                        .expect("Could not send value")
                });
            }
        });

        Some(inner_guard)
    }

    fn try_write_untracked(&self) -> Option<impl DerefMut<Target = Self::Value>> {
        let mut guard = self.try_write()?;
        guard.untrack();
        Some(guard)
    }
}

// TODO : make sync_resource out of this with an `Into<UseRwSignal>` as input.
/// Creates a pair of a `RwSignalResource` and a `BevyEventDuplex`.
///
/// The first can be used just like a `RwSignal` in Leptos. The `BevyEventDuplex` that has to
/// be passed into the Bevy app where it will be used to sync the signal with a Bevy `Resource`.
pub fn signal_resource<T>(initial_value: T) -> (RwSignalResource<T>, BevyEventDuplex<T>)
where
    T: Send + Sync + Clone + 'static,
{
    let (tx_l2b, rx_l2b) = crossbeam_channel::bounded(50);
    let (tx_b2l, rx_b2l) = crossbeam_channel::bounded(50);

    tx_l2b
        .send(initial_value.clone())
        .expect("Could not send initial value");

    let signal = RwSignal::new(initial_value);

    use_raf_fn({
        let rx = rx_b2l.clone();

        move |_| {
            for event in rx.try_iter() {
                signal.set(event);
            }
        }
    });

    (
        RwSignalResource {
            rw_signal: signal,
            tx: StoredValue::new(tx_l2b),
        },
        BevyEventDuplex::new(rx_l2b, tx_b2l),
    )
}
