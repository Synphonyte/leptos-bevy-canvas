mod bevy;
mod leptos;

pub use crate::messages::bevy::*;
pub use crate::messages::leptos::*;
use crate::utils::init_rw_signal_from_receiver;

/// Creates a pair of a `LeptosMessageSender` and a `BevyMessageReceiver`.
///
/// The `LeptosMessageSender` can be used in the Leptos app to send messages to Bevy.
/// The `BevyMessageReceiver` has to be passed to the Bevy app to receive these messages in the form
/// of normal Bevy messages.
pub fn message_l2b<M>() -> (LeptosMessageSender<M>, BevyMessageReceiver<M>)
where
    M: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    (LeptosMessageSender::new(tx), BevyMessageReceiver::new(rx))
}

/// Creates a pair of a `LeptosMessageReceiver` and a `BevyMessageSender`.
///
/// The `LeptosMessageReceiver` can be used in the Leptos app like a normal `Signal` to read the
/// latest message that was received from Bevy.
/// The `BevyMessageSender` has to be passed to the Bevy app so whenever you write the specified
/// message in the Bevy app with an message writer it will be sent to the Leptos signal.
pub fn message_b2l<M>() -> (LeptosMessageReceiver<M>, BevyMessageSender<M>)
where
    M: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx);

    (
        LeptosMessageReceiver::new(rx, signal),
        BevyMessageSender::new(tx),
    )
}

/// Combines the functionality of `message_l2b` and `message_b2l` to send and receive messages in
/// both directions.
pub fn message_duplex<M>() -> (LeptosMessageDuplex<M>, BevyMessageDuplex<M>)
where
    M: Send + Sync + 'static,
{
    let (tx_l2b, rx_l2b) = crossbeam_channel::bounded(50);
    let (tx_b2l, rx_b2l) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx_b2l);

    (
        LeptosMessageDuplex::new(rx_b2l, signal, tx_l2b),
        BevyMessageDuplex::new(rx_l2b, tx_b2l),
    )
}
