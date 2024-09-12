use crossbeam_channel::{Receiver, Sender};

pub trait HasReceiver<T> {
    fn rx(&self) -> &Receiver<T>;
}

pub trait HasSender<T> {
    fn tx(&self) -> &Sender<T>;
}
