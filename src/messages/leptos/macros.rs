macro_rules! impl_read_signal {
    ($name:ident) => {
        impl<M> DefinedAt for $name<M>
        where
            M: Send + Sync + 'static,
        {
            fn defined_at(&self) -> Option<&'static Location<'static>> {
                self.rx_signal.defined_at()
            }
        }

        impl<M> IsDisposed for $name<M>
        where
            M: Send + Sync + 'static,
        {
            fn is_disposed(&self) -> bool {
                self.rx_signal.is_disposed()
            }
        }

        impl<M> ReadUntracked for $name<M>
        where
            M: Send + Sync + 'static,
        {
            type Value = ReadGuard<Option<M>, Plain<Option<M>>>;

            fn try_read_untracked(&self) -> Option<Self::Value> {
                self.rx_signal.try_read_untracked()
            }
        }

        impl<M> Track for $name<M>
        where
            M: Send + Sync + 'static,
        {
            fn track(&self) {
                self.rx_signal.track()
            }
        }
    };
}

pub(super) use impl_read_signal;
