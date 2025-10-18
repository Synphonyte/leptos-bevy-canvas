mod macros;

use crate::traits::{HasReceiver, HasSender};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::messages::bevy::macros::{impl_has_receiver, impl_has_sender};

/// This is passed to Bevy to receive messages from the Leptos app.
#[derive(Resource)]
pub struct BevyMessageReceiver<M> {
    rx: Receiver<M>,
}

impl<M> Clone for BevyMessageReceiver<M> {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
        }
    }
}

impl<M> std::fmt::Debug for BevyMessageReceiver<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyMessageReceiver");
        s.field("rx", &self.rx);
        s.finish()
    }
}

impl<M> BevyMessageReceiver<M> {
    #[inline]
    pub fn new(rx: Receiver<M>) -> Self {
        Self { rx }
    }
}

impl_has_receiver!(BevyMessageReceiver);

/// This is passed to Bevy to send messages to the Leptos app.
#[derive(Resource)]
pub struct BevyMessageSender<M> {
    tx: Sender<M>,
}

impl<M> Clone for BevyMessageSender<M> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<M> std::fmt::Debug for BevyMessageSender<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyMessageSender");
        s.field("tx", &self.tx);
        s.finish()
    }
}

impl<M> BevyMessageSender<M> {
    #[inline]
    pub fn new(tx: Sender<M>) -> Self {
        Self { tx }
    }
}

impl_has_sender!(BevyMessageSender);

/// This is passed to Bevy to send and receive messages in both directions.
#[derive(Resource)]
pub struct BevyMessageDuplex<M> {
    tx: Sender<M>,
    rx: Receiver<M>,
}

impl<M> Clone for BevyMessageDuplex<M> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<M> std::fmt::Debug for BevyMessageDuplex<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BevyMessageDuplex");
        s.field("tx", &self.tx);
        s.field("rx", &self.rx);
        s.finish()
    }
}

impl<M> BevyMessageDuplex<M> {
    #[inline]
    pub fn new(rx: Receiver<M>, tx: Sender<M>) -> Self {
        Self { tx, rx }
    }
}

impl_has_receiver!(BevyMessageDuplex);
impl_has_sender!(BevyMessageDuplex);
