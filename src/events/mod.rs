mod bevy;
mod leptos;

pub use crate::events::bevy::*;
pub use crate::events::leptos::*;
use crate::utils::init_rw_signal_from_receiver;

/// Creates a pair of a `LeptosEventSender` and a `BevyEventReceiver`.
/// The `LeptosEventSender` can be used in the Leptos app to send events to Bevy.
/// The `BevyEventReceiver` has to be passed to the Bevy app to receive these events in the form
/// of normal Bevy events.
pub fn event_l2b<E>() -> (LeptosEventSender<E>, BevyEventReceiver<E>)
where
    E: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    (LeptosEventSender::new(tx), BevyEventReceiver::new(rx))
}

/// Creates a pair of a `LeptosEventReceiver` and a `BevyEventSender`.
/// The `LeptosEventReceiver` can be used in the Leptos app like a normal `Signal` to read the
/// latest event that was received from Bevy.
/// The `BevyEventSender` has to be passed to the Bevy app so whenever you write the specified
/// event in the Bevy app with an event writer it will be sent to the Leptos signal.
pub fn event_b2l<E>() -> (LeptosEventReceiver<E>, BevyEventSender<E>)
where
    E: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx);

    (
        LeptosEventReceiver::new(rx, signal),
        BevyEventSender::new(tx),
    )
}

/// Combines the functionality of `event_l2b` and `event_b2l` to send and receive events in
/// both directions.
pub fn event_duplex<E>() -> (LeptosEventDuplex<E>, BevyEventDuplex<E>)
where
    E: Send + Sync + 'static,
{
    let (tx_l2b, rx_l2b) = crossbeam_channel::bounded(50);
    let (tx_b2l, rx_b2l) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx_b2l);

    (
        LeptosEventDuplex::new(rx_b2l, signal, tx_l2b),
        BevyEventDuplex::new(rx_l2b, tx_b2l),
    )
}
