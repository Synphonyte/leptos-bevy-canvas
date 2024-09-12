mod macros;
mod traits;

use crate::events::leptos::macros::impl_read_signal;
use crossbeam_channel::{Receiver, Sender};
use leptos::prelude::guards::{Plain, ReadGuard};
use leptos::prelude::*;
use std::panic::Location;

pub use self::traits::*;

#[derive(Copy)]
pub struct LeptosEventSender<E>
where
    E: Send + Sync + 'static,
{
    tx: StoredValue<Sender<E>>,
}

impl<E> Clone for LeptosEventSender<E>
where
    E: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { tx: self.tx }
    }
}

impl<E> LeptosChannelEventSender for LeptosEventSender<E>
where
    E: Send + Sync + 'static,
{
    type Event = E;

    fn tx(&self) -> StoredValue<Sender<Self::Event>> {
        self.tx
    }
}

impl<E> LeptosEventSender<E>
where
    E: Send + Sync + 'static,
{
    pub fn new(tx: Sender<E>) -> Self {
        Self {
            tx: StoredValue::new(tx),
        }
    }
}

#[derive(Copy)]
pub struct LeptosEventReceiver<E>
where
    E: Send + Sync + 'static,
{
    rx: StoredValue<Receiver<E>>,
    rx_signal: RwSignal<Option<E>>,
}

impl<E> Clone for LeptosEventReceiver<E>
where
    E: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            rx: self.rx,
            rx_signal: self.rx_signal,
        }
    }
}

impl_read_signal!(LeptosEventReceiver);

impl<E> LeptosEventReceiver<E>
where
    E: Send + Sync + 'static,
{
    #[inline]
    pub fn new(rx: Receiver<E>, signal: RwSignal<Option<E>>) -> Self {
        Self {
            rx: StoredValue::new(rx),
            rx_signal: signal,
        }
    }
}

#[derive(Copy)]
pub struct LeptosEventDuplex<E>
where
    E: Send + Sync + 'static,
{
    tx: StoredValue<Sender<E>>,
    rx: StoredValue<Receiver<E>>,
    rx_signal: RwSignal<Option<E>>,
}

impl<E> Clone for LeptosEventDuplex<E>
where
    E: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx,
            rx: self.rx,
            rx_signal: self.rx_signal,
        }
    }
}

impl<E> LeptosChannelEventSender for LeptosEventDuplex<E>
where
    E: Send + Sync + 'static,
{
    type Event = E;

    #[inline]
    fn tx(&self) -> StoredValue<Sender<Self::Event>> {
        self.tx
    }
}

impl_read_signal!(LeptosEventDuplex);

impl<E> LeptosEventDuplex<E>
where
    E: Send + Sync + 'static,
{
    #[inline]
    pub fn new(rx: Receiver<E>, rx_signal: RwSignal<Option<E>>, tx: Sender<E>) -> Self {
        Self {
            tx: StoredValue::new(tx),
            rx: StoredValue::new(rx),
            rx_signal,
        }
    }
}
