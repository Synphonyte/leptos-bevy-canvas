use crate::events::BevyEventDuplex;
use crate::prelude::QueryDataOwned;
use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::event::EventId;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncSignalResourceSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ImportLeptosEventSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ExportLeptosEventSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncQuerySet;

/// Keeps track of what Leptos event have been imported into Bevy to prevent infinite loops.
#[derive(Resource, Deref, DerefMut)]
pub struct ImportedEventIds<E: Event>(Vec<EventId<E>>);

impl<E: Event> Default for ImportedEventIds<E> {
    fn default() -> Self {
        Self(Vec::with_capacity(4))
    }
}

/// Imports an event from Leptos and writes it as a Bevy event.
pub fn import_and_send_leptos_events<R, E>(
    rx: Res<R>,
    mut imported_event_ids: ResMut<ImportedEventIds<E>>,
    mut event_writer: EventWriter<E>,
) where
    R: HasReceiver<E> + Resource,
    E: Event,
{
    imported_event_ids.clear();

    for event in rx.rx().try_iter() {
        let event_id = event_writer.write(event);
        imported_event_ids.push(event_id);
    }
}

/// Exports an event from Bevy to Leptos.
pub fn read_and_export_leptos_events<S, E>(
    tx: Res<S>,
    imported_event_ids: Res<ImportedEventIds<E>>,
    mut event_reader: EventReader<E>,
) where
    S: HasSender<E> + Resource,
    E: Event + Clone,
{
    for (event, id) in event_reader.read_with_id() {
        if !imported_event_ids.contains(&id) {
            tx.tx().send(event.clone()).unwrap();
        }
    }
}

/// Takes care of synchronizing a resource between Bevy and a Leptos signal
pub fn sync_signal_resource<D, R>(mut resource: ResMut<R>, sync: Res<D>)
where
    R: Resource + Clone,
    D: HasReceiver<R> + HasSender<R> + Resource,
{
    if resource.is_changed() && !resource.is_added() {
        sync.tx().send(resource.clone()).unwrap();
    }

    for event in sync.rx().try_iter() {
        *resource = event;
    }
}

/// Synchronizes a Bevy query's `.get_single_mut()` with a Leptos signal.
pub fn sync_query<D, F>(
    duplex: Res<BevyEventDuplex<Option<D>>>,
    mut query: Query<<D as QueryDataOwned>::Qdata, F>,
    mut prev_some: Local<bool>,
) where
    for<'a> D: QueryDataOwned<'a> + Send + Sync + 'static,
    F: QueryFilter,
{
    let mut item = query.single_mut().ok();

    let changed = if let Some(item) = &item {
        !*prev_some || D::is_changed(item)
    } else {
        *prev_some
    };

    *prev_some = item.is_some();

    if changed {
        let item = item.map(|item| D::from_query_data(&item));
        duplex.tx().send(item).unwrap();
    } else {
        for event in duplex.rx().try_iter() {
            if let (Some(event), Some(item)) = (event, &mut item) {
                event.set_query_data(item);
            }
        }
    }
}
