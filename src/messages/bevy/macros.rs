macro_rules! impl_has_receiver {
    ($name:ident) => {
        impl<T> HasReceiver<T> for $name<T> {
            fn rx(&self) -> &crossbeam_channel::Receiver<T> {
                &self.rx
            }
        }
    };
}

macro_rules! impl_has_sender {
    ($name:ident) => {
        impl<T> HasSender<T> for $name<T> {
            fn tx(&self) -> &crossbeam_channel::Sender<T> {
                &self.tx
            }
        }
    };
}

pub(super) use impl_has_receiver;
pub(super) use impl_has_sender;
