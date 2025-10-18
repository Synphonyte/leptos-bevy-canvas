mod macros;
mod traits;

use crate::messages::leptos::macros::impl_read_signal;
use crossbeam_channel::{Receiver, Sender};
use leptos::prelude::guards::{Plain, ReadGuard};
use leptos::prelude::*;
use std::panic::Location;

pub use self::traits::*;

/// This is a Leptos message sender that can be used to send messages to Bevy.
/// It provides a `send` method to do this.
#[derive(Copy)]
pub struct LeptosMessageSender<M>
where
    M: Send + Sync + 'static,
{
    tx: StoredValue<Sender<M>>,
}

impl<M> Clone for LeptosMessageSender<M>
where
    M: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { tx: self.tx }
    }
}

impl<M> LeptosChannelMessageSender for LeptosMessageSender<M>
where
    M: Send + Sync + 'static,
{
    type Message = M;

    fn tx(&self) -> StoredValue<Sender<Self::Message>> {
        self.tx
    }
}

impl<M> LeptosMessageSender<M>
where
    M: Send + Sync + 'static,
{
    pub fn new(tx: Sender<M>) -> Self {
        Self {
            tx: StoredValue::new(tx),
        }
    }
}

/// This is a Leptos message receiver that can be used to receive messages from Bevy.
/// This can be used just like a normal Leptos `Signal` to read the latest message.
#[derive(Copy)]
pub struct LeptosMessageReceiver<M>
where
    M: Send + Sync + 'static,
{
    rx: StoredValue<Receiver<M>>,
    rx_signal: RwSignal<Option<M>>,
}

impl<M> Clone for LeptosMessageReceiver<M>
where
    M: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            rx: self.rx,
            rx_signal: self.rx_signal,
        }
    }
}

impl_read_signal!(LeptosMessageReceiver);

impl<M> LeptosMessageReceiver<M>
where
    M: Send + Sync + 'static,
{
    #[inline]
    pub fn new(rx: Receiver<M>, signal: RwSignal<Option<M>>) -> Self {
        Self {
            rx: StoredValue::new(rx),
            rx_signal: signal,
        }
    }
}

/// Combines the functionality of `LeptosMessageSender` and `LeptosMessageReceiver`.
#[derive(Copy)]
pub struct LeptosMessageDuplex<M>
where
    M: Send + Sync + 'static,
{
    tx: StoredValue<Sender<M>>,
    rx: StoredValue<Receiver<M>>,
    rx_signal: RwSignal<Option<M>>,
}

impl<M> Clone for LeptosMessageDuplex<M>
where
    M: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx,
            rx: self.rx,
            rx_signal: self.rx_signal,
        }
    }
}

impl<M> LeptosChannelMessageSender for LeptosMessageDuplex<M>
where
    M: Send + Sync + 'static,
{
    type Message = M;

    #[inline]
    fn tx(&self) -> StoredValue<Sender<Self::Message>> {
        self.tx
    }
}

impl_read_signal!(LeptosMessageDuplex);

impl<M> LeptosMessageDuplex<M>
where
    M: Send + Sync + 'static,
{
    #[inline]
    pub fn new(rx: Receiver<M>, rx_signal: RwSignal<Option<M>>, tx: Sender<M>) -> Self {
        Self {
            tx: StoredValue::new(tx),
            rx: StoredValue::new(rx),
            rx_signal,
        }
    }
}
