use crossbeam_channel::{SendError, Sender};
use leptos::prelude::*;

pub trait LeptosChannelEventSender {
    type Event: Send + Sync + 'static;

    fn tx(&self) -> StoredValue<Sender<Self::Event>>;

    #[inline]
    fn send(&self, event: Self::Event) -> Result<(), SendError<Self::Event>> {
        self.tx().with_value(|tx| tx.send(event))
    }
}
