mod macros;

use crate::traits::{HasReceiver, HasSender};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::events::bevy::macros::{impl_has_receiver, impl_has_sender};

/// This is passed to Bevy to receive events from the Leptos app.
#[derive(Resource)]
pub struct BevyEventReceiver<E> {
    rx: Receiver<E>,
}

impl<E> Clone for BevyEventReceiver<E> {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
        }
    }
}

impl<E> std::fmt::Debug for BevyEventReceiver<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyEventReceiver");
        s.field("rx", &self.rx);
        s.finish()
    }
}

impl<E> BevyEventReceiver<E> {
    #[inline]
    pub fn new(rx: Receiver<E>) -> Self {
        Self { rx }
    }
}

impl_has_receiver!(BevyEventReceiver);

/// This is passed to Bevy to send events to the Leptos app.
#[derive(Resource)]
pub struct BevyEventSender<E> {
    tx: Sender<E>,
}

impl<E> Clone for BevyEventSender<E> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<E> std::fmt::Debug for BevyEventSender<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyEventSender");
        s.field("tx", &self.tx);
        s.finish()
    }
}

impl<E> BevyEventSender<E> {
    #[inline]
    pub fn new(tx: Sender<E>) -> Self {
        Self { tx }
    }
}

impl_has_sender!(BevyEventSender);

/// This is passed to Bevy to send and receive events in both directions.
#[derive(Resource)]
pub struct BevyEventDuplex<E> {
    tx: Sender<E>,
    rx: Receiver<E>,
}

impl<E> Clone for BevyEventDuplex<E> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<E> std::fmt::Debug for BevyEventDuplex<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyEventDuplex");
        s.field("tx", &self.tx);
        s.field("rx", &self.rx);
        s.finish()
    }
}

impl<E> BevyEventDuplex<E> {
    #[inline]
    pub fn new(rx: Receiver<E>, tx: Sender<E>) -> Self {
        Self { tx, rx }
    }
}

impl_has_receiver!(BevyEventDuplex);
impl_has_sender!(BevyEventDuplex);
