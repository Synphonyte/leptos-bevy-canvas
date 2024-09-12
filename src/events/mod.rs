mod bevy;
mod leptos;

pub use crate::events::bevy::*;
pub use crate::events::leptos::*;
use crate::utils::init_rw_signal_from_receiver;

pub fn event_l2b<E>() -> (LeptosEventSender<E>, BevyEventReceiver<E>)
where
    E: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    (LeptosEventSender::new(tx), BevyEventReceiver::new(rx))
}

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
