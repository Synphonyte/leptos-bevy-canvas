use bevy::{asset::UntypedAssetId, platform::collections::HashSet, prelude::*};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AssetsLoading(HashSet<UntypedAssetId>);
