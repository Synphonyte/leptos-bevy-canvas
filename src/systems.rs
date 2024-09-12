use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::event::EventId;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncSignalResourceSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ImportLeptosEventSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ExportLeptosEventSet;

#[derive(Resource, Deref, DerefMut)]
pub struct ImportedEventIds<E: Event>(Vec<EventId<E>>);

impl<E: Event> Default for ImportedEventIds<E> {
    fn default() -> Self {
        Self(Vec::with_capacity(4))
    }
}

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
        let event_id = event_writer.send(event);
        imported_event_ids.push(event_id);
    }
}

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

pub fn sync_signal_resource<D, R>(mut resource: ResMut<R>, sync: Res<D>)
where
    R: Resource + Clone,
    D: HasReceiver<R> + HasSender<R> + Resource,
{
    if resource.is_changed() {
        sync.tx().send(resource.clone()).unwrap();
    }

    for event in sync.rx().try_iter() {
        *resource = event;
    }
}
