use crossbeam_channel::{SendError, Sender};
use leptos::prelude::*;

/// This is a trait that is implemented by a Leptos message sender.
pub trait LeptosChannelMessageSender {
    type Message: Send + Sync + 'static;

    fn tx(&self) -> StoredValue<Sender<Self::Message>>;

    /// Call this to send an message to the Bevy app.
    #[inline]
    fn send(&self, message: Self::Message) -> Result<(), SendError<Self::Message>> {
        self.tx().with_value(|tx| tx.send(message))
    }
}
