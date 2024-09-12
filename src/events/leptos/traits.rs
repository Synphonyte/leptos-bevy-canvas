use crossbeam_channel::{SendError, Sender};
use leptos::prelude::*;

/// This is a trait that is implemented by a Leptos event sender.
pub trait LeptosChannelEventSender {
    type Event: Send + Sync + 'static;

    fn tx(&self) -> StoredValue<Sender<Self::Event>>;

    /// Call this to send an event to the Bevy app.
    #[inline]
    fn send(&self, event: Self::Event) -> Result<(), SendError<Self::Event>> {
        self.tx().with_value(|tx| tx.send(event))
    }
}
