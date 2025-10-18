use crate::bevy_app::components::Model;
use crate::bevy_app::resources::AssetsLoading;
use crate::bevy_app::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn track_assets_loading(
    asset_server: Res<AssetServer>,
    assets_loading: Res<AssetsLoading>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for asset_id in &**assets_loading {
        if !matches!(
            asset_server.get_load_state(*asset_id),
            Some(LoadState::Loaded)
        ) {
            return;
        }
    }

    next_state.set(AppState::Ready);
}

pub fn toogle_between_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    active_model_query: Query<(Entity, &Model)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    leptos::logging::log!("1");
    next_state.set(AppState::Loading);
    leptos::logging::log!("2");

    // Load next model
    let next_model = if let Ok((entity, model)) = active_model_query.single() {
        commands.entity(entity).despawn();

        model.next()
    } else {
        Model::Robot
    };
    leptos::logging::log!("3");

    let next_url = format!(
        "https://media.githubusercontent.com/media/Synphonyte/leptos-bevy-canvas/main/examples/loading_screen/assets/models/{}",
        next_model.src()
    );
    let next_model_asset_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(next_url));
    leptos::logging::log!("4");

    assets_loading.insert(next_model_asset_handle.id().into());

    leptos::logging::log!("5");

    commands.spawn((
        SceneRoot(next_model_asset_handle),
        next_model.initial_transform(),
        next_model,
    ));

    leptos::logging::log!("6");
}
