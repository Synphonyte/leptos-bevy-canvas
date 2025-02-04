use crate::events::BevyEventDuplex;
use crate::signal_synced::{signal_synced, RwSignalSynced};
use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::prelude::*;
use bevy::utils::all_tuples;
use paste::paste;
use std::marker::PhantomData;

/// `RwSignal` like synchronization for bevy queries.
///
/// Creates a pair of a `RwSignalSynced` and a `BevyQueryDuplex` for a bevy query that is
/// evaluated as `.get_single_mut()`.
///
/// ## Example
///
/// ```
/// # use bevy::prelude::*;
/// # use leptos_bevy_canvas::prelude::single_query_signal;
/// #
/// # #[derive(Component)]
/// # struct Selected;
///
/// let (selected, selected_query_duplex) = single_query_signal::<(Transform,), With<Selected>>();
/// ```
pub fn single_query_signal<D, F>() -> (RwSignalSynced<Option<D>>, BevyQueryDuplex<D, F>)
where
    for<'a> D: QueryDataOwned<'a> + Clone + Send + Sync + 'static,
    F: QueryFilter,
{
    let (signal, duplex) = signal_synced(None);

    (
        signal,
        BevyQueryDuplex {
            duplex,
            marker: PhantomData,
        },
    )
}

pub trait QueryDataOwned<'q> {
    type Qdata: QueryData + WorldQuery;

    fn from_query_data<'a>(data: &<Self::Qdata as WorldQuery>::Item<'a>) -> Self;

    fn set_query_data<'a>(&self, data: &mut <Self::Qdata as WorldQuery>::Item<'a>);

    fn is_changed<'a>(data: &<Self::Qdata as WorldQuery>::Item<'a>) -> bool;
}

macro_rules! impl_as_query_data {
    ($(#[$meta:meta])* $($name:ident),*) => {
        $(#[$meta])*
        impl<'q, $($name: bevy::prelude::Component + Clone),*> QueryDataOwned<'q> for ($($name,)*) {
            type Qdata = ($(&'q mut $name,)*);

            fn from_query_data<'a>(data: &<Self::Qdata as WorldQuery>::Item<'a>) -> Self {
                paste! {
                    let ($([<$name:lower>],)*) = data;
                    ($(
                        (**[<$name:lower>]).clone(),
                    )*)
                }
            }

            fn set_query_data<'a>(&self, data: &mut <Self::Qdata as WorldQuery>::Item<'a>) {
                paste! {
                    let ($([<$name:lower>],)*) = data;
                    let ($([<$name:lower _self>],)*) = self;

                    $(
                        **[<$name:lower>] = (*[<$name:lower _self>]).clone();
                    )*
                }
            }

            fn is_changed<'a>(data: &<Self::Qdata as WorldQuery>::Item<'a>) -> bool {
                paste! {
                    let ($([<$name:lower>],)*) = data;
                    $(
                        if [<$name:lower>].is_changed() {
                            return true;
                        }
                    )*
                    false
                }
            }
        }
    };
}

all_tuples!(impl_as_query_data, 1, 15, T);

pub struct BevyQueryDuplex<D, F = ()>
where
    for<'a> D: QueryDataOwned<'a>,
    F: QueryFilter,
{
    pub(crate) duplex: BevyEventDuplex<Option<D>>,
    marker: PhantomData<F>,
}

impl<D, F> Clone for BevyQueryDuplex<D, F>
where
    for<'a> D: QueryDataOwned<'a>,
    F: QueryFilter,
{
    fn clone(&self) -> Self {
        Self {
            duplex: self.duplex.clone(),
            marker: PhantomData,
        }
    }
}
