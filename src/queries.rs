use crate::events::BevyEventDuplex;
use crate::signal_synced::{signal_synced, RwSignalSynced};
use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;
use bevy::utils::all_tuples;
use leptos::prelude::*;
use paste::paste;
use std::marker::PhantomData;

pub struct BevyQuery<D: QueryData, F: QueryFilter = ()>(PhantomData<(D, F)>);

impl<D: QueryData, F: QueryFilter> Clone for BevyQuery<D, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: QueryData, F: QueryFilter> Copy for BevyQuery<D, F> {}

impl<D, F, CI> BevyQuery<D, F>
where
    D: QueryData,
    F: QueryFilter,
    CI: Clone + Send + Sync + 'static,
    for<'i> D::Item<'i>: CloneItem<Output = CI>,
{
    #[inline(always)]
    pub fn signal() -> (RwSignalSynced<Option<CI>>, BevyEventDuplex<Option<CI>>) {
        signal_synced(None)
    }
}

pub trait CloneItem {
    type Output;

    fn clone_item(&self) -> Self::Output;
}

impl<T: Clone> CloneItem for Mut<'_, T> {
    type Output = T;

    fn clone_item(&self) -> Self::Output {
        (*self).clone()
    }
}

macro_rules! impl_clone_item {
    ($(#[$meta:meta])* $($name:ident),*) => {
        #[allow(clippy::unused_unit)]
        $(#[$meta])*
        impl<$($name: Clone),*> CloneItem for Mut<'_, ($(&$name,)*)> {
            type Output = ($($name,)*);

            fn clone_item(&self) -> Self::Output {
                paste! {
                    let ($([<$name:lower>],)*) = **self;

                    ($(
                        [<$name:lower>].clone(),
                    )*)
                }
            }
        }
    };
}

// all_tuples!(impl_clone_item, 1, 15, T);

pub trait SetItem<T> {
    fn set_item(&self, item: T);
}

impl<T: Clone> SetItem<&mut Mut<'_, T>> for T {
    fn set_item(&self, item: &mut Mut<'_, T>) {
        **item = self.clone();
    }
}

macro_rules! impl_set_item {
    ($(#[$meta:meta])* $($name:ident),*) => {
        #[allow(clippy::unused_unit)]
        $(#[$meta])*
        impl<$($name: Clone),*> SetItem<($(&mut $name,)*)> for ($($name,)*) {
            fn set_item(&self, item: ($(&mut $name,)*)) {
                paste! {
                    let ($([<$name:lower>],)*) = item;
                    let ($([<$name:lower _self>],)*) = self.clone();

                    $(
                        *[<$name:lower>] = [<$name:lower _self>];
                    )*
                }
            }
        }
    };
}

all_tuples!(impl_set_item, 1, 15, T);
