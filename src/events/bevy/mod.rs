mod macros;

use crate::traits::{HasReceiver, HasSender};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::events::bevy::macros::{impl_has_receiver, impl_has_sender};

#[derive(Resource)]
pub struct BevyEventReceiver<E> {
    rx: Receiver<E>,
}

impl<E> BevyEventReceiver<E> {
    #[inline]
    pub fn new(rx: Receiver<E>) -> Self {
        Self { rx }
    }
}

impl_has_receiver!(BevyEventReceiver);

#[derive(Resource)]
pub struct BevyEventSender<E> {
    tx: Sender<E>,
}

impl<E> BevyEventSender<E> {
    #[inline]
    pub fn new(tx: Sender<E>) -> Self {
        Self { tx }
    }
}

impl_has_sender!(BevyEventSender);

#[derive(Resource)]
pub struct BevyEventDuplex<E> {
    tx: Sender<E>,
    rx: Receiver<E>,
}

impl<E> BevyEventDuplex<E> {
    #[inline]
    pub fn new(rx: Receiver<E>, tx: Sender<E>) -> Self {
        Self { tx, rx }
    }
}

impl_has_receiver!(BevyEventDuplex);
impl_has_sender!(BevyEventDuplex);
