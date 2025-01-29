use crate::events::BevyEventDuplex;
use crate::prelude::{CloneItem, SetItem};
use crate::systems::*;
use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::prelude::*;

/// Adds synchronization methods to the Bevy app
pub trait LeptosBevyApp {
    /// Imports an event from Leptos into the Bevy app. Takes the Bevy event receiver as argument.
    fn import_event_from_leptos<R, E>(&mut self, bevy_rx: R) -> &mut Self
    where
        E: Event,
        R: HasReceiver<E> + Resource;

    /// Exports an event from Bevy to Leptos. Takes the Bevy event sender as argument.
    fn export_event_to_leptos<S, E>(&mut self, bevy_tx: S) -> &mut Self
    where
        E: Event + Clone,
        S: HasSender<E> + Resource;

    /// Adds duplex event handling between Bevy and Leptos. Takes the Bevy event receiver/sender as argument.
    fn add_duplex_leptos_event<D, E>(&mut self, bevy_duplex: D) -> &mut Self
    where
        E: Event + Clone,
        D: HasReceiver<E> + HasSender<E> + Resource;

    /// Adds resource syncing between Bevy and Leptos. Takes the Bevy resource receiver/sender as argument.
    fn sync_leptos_signal_with_resource<D, R>(&mut self, bevy_duplex: D) -> &mut Self
    where
        R: Resource + Clone,
        D: HasReceiver<R> + HasSender<R> + Resource;

    fn sync_leptos_signal_with_query<D, F, CI>(&mut self, duplex: BevyEventDuplex<Option<CI>>) -> &mut Self
    where
        D: QueryData + 'static,
        for<'i> D::Item<'i>: CloneItem<Output = CI> + DetectChanges,
        F: QueryFilter + 'static,
        for<'a, 'i> CI: SetItem<&'a mut D::Item<'i>>,
        CI: Clone + Send + 'static;
}

impl LeptosBevyApp for App {
    fn import_event_from_leptos<R, E>(&mut self, bevy_rx: R) -> &mut Self
    where
        E: Event,
        R: HasReceiver<E> + Resource,
    {
        self.insert_resource(bevy_rx)
            .add_event::<E>()
            .init_resource::<ImportedEventIds<E>>()
            .add_systems(
                PreUpdate,
                import_and_send_leptos_events::<R, E>.in_set(ImportLeptosEventSet),
            )
    }

    fn export_event_to_leptos<R, E>(&mut self, bevy_tx: R) -> &mut Self
    where
        E: Event + Clone,
        R: HasSender<E> + Resource,
    {
        self.insert_resource(bevy_tx)
            .add_event::<E>()
            .init_resource::<ImportedEventIds<E>>()
            .add_systems(
                PostUpdate,
                read_and_export_leptos_events::<R, E>.in_set(ExportLeptosEventSet),
            )
    }

    fn add_duplex_leptos_event<D, E>(&mut self, bevy_duplex: D) -> &mut Self
    where
        E: Event + Clone,
        D: HasReceiver<E> + HasSender<E> + Resource,
    {
        self.insert_resource(bevy_duplex)
            .add_event::<E>()
            .add_systems(
                PreUpdate,
                import_and_send_leptos_events::<D, E>.in_set(ImportLeptosEventSet),
            )
            .add_systems(
                PostUpdate,
                read_and_export_leptos_events::<D, E>.in_set(ExportLeptosEventSet),
            )
    }

    fn sync_leptos_signal_with_resource<D, R>(&mut self, bevy_duplex: D) -> &mut Self
    where
        R: Resource + Clone,
        D: HasReceiver<R> + HasSender<R> + Resource,
    {
        for event in bevy_duplex.rx().try_iter() {
            self.insert_resource(event);
        }

        self.insert_resource(bevy_duplex).add_systems(
            Update,
            sync_signal_resource::<D, R>.in_set(SyncSignalResourceSet),
        )
    }

    fn sync_leptos_signal_with_query<D, F, CI>(&mut self, duplex: BevyEventDuplex<Option<CI>>) -> &mut Self
    where
        D: QueryData + 'static,
        for<'i> D::Item<'i>: CloneItem<Output = CI> + DetectChanges,
        F: QueryFilter + 'static,
        for<'a, 'i> CI: SetItem<&'a mut D::Item<'i>>,
        CI: Clone + Send + 'static,
    {
        self.insert_resource(duplex)
            .add_systems(Update, sync_query::<D, F, CI>.in_set(SyncQuerySet))
    }
}
