use crate::messages::BevyMessageDuplex;
use crate::prelude::QueryDataOwned;
use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::message::MessageId;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncSignalResourceSet;

#[cfg(feature = "bevy_state")]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncSignalStateSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ImportLeptosMessageSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ExportLeptosMessageSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SyncQuerySet;

/// Keeps track of what Leptos message have been imported into Bevy to prevent infinite loops.
#[derive(Resource, Deref, DerefMut)]
pub struct ImportedMessageIds<M: Message>(Vec<MessageId<M>>);

impl<M: Message> Default for ImportedMessageIds<M> {
    fn default() -> Self {
        Self(Vec::with_capacity(4))
    }
}

/// Imports a message from Leptos and writes it as a Bevy message.
pub fn import_and_send_leptos_messages<R, M>(
    rx: Res<R>,
    mut imported_message_ids: ResMut<ImportedMessageIds<M>>,
    mut message_writer: MessageWriter<M>,
) where
    R: HasReceiver<M> + Resource,
    M: Message,
{
    imported_message_ids.clear();

    for message in rx.rx().try_iter() {
        let message_id = message_writer.write(message);
        imported_message_ids.push(message_id);
    }
}

/// Exports a message from Bevy to Leptos.
pub fn read_and_export_leptos_messages<S, M>(
    tx: Res<S>,
    imported_message_ids: Res<ImportedMessageIds<M>>,
    mut message_reader: MessageReader<M>,
) where
    S: HasSender<M> + Resource,
    M: Message + Clone,
{
    for (message, id) in message_reader.read_with_id() {
        if !imported_message_ids.contains(&id) {
            tx.tx().send(message.clone()).unwrap();
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

/// Takes care of synchronizing a state between Bevy and a Leptos signal
#[cfg(feature = "bevy_state")]
pub fn sync_signal_state<D, S>(mut state: ResMut<State<S>>, sync: Res<D>)
where
    S: bevy::state::state::FreelyMutableState + Clone,
    D: HasReceiver<S> + HasSender<S> + Resource,
{
    if state.is_changed() && !state.is_added() {
        sync.tx().send(state.clone()).unwrap();
    }

    for event in sync.rx().try_iter() {
        *state = State::new(event);
    }
}

/// Synchronizes a Bevy query's `.get_single_mut()` with a Leptos signal.
pub fn sync_query<D, F>(
    duplex: Res<BevyMessageDuplex<Option<D>>>,
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
        for message in duplex.rx().try_iter() {
            if let (Some(message), Some(item)) = (message, &mut item) {
                message.set_query_data(item);
            }
        }
    }
}
