macro_rules! impl_read_signal {
    ($name:ident) => {
        impl<E> DefinedAt for $name<E>
        where
            E: Send + Sync + 'static,
        {
            fn defined_at(&self) -> Option<&'static Location<'static>> {
                self.rx_signal.defined_at()
            }
        }

        impl<E> IsDisposed for $name<E>
        where
            E: Send + Sync + 'static,
        {
            fn is_disposed(&self) -> bool {
                self.rx_signal.is_disposed()
            }
        }

        impl<E> ReadUntracked for $name<E>
        where
            E: Send + Sync + 'static,
        {
            type Value = ReadGuard<Option<E>, Plain<Option<E>>>;

            fn try_read_untracked(&self) -> Option<Self::Value> {
                self.rx_signal.try_read_untracked()
            }
        }

        impl<E> Track for $name<E>
        where
            E: Send + Sync + 'static,
        {
            fn track(&self) {
                self.rx_signal.track()
            }
        }
    };
}

pub(super) use impl_read_signal;
