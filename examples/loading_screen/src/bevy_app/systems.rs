use crate::bevy_app::components::Model;
use crate::bevy_app::AppState;
use bevy::prelude::*;
use iyes_progress::AssetsLoading;

pub fn toogle_between_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading<AppState>>,
    active_model_query: Query<(Entity, &Model), With<Model>>,
) {
    commands.set_state(AppState::Loading);

    let next_model = if let Ok(active_model) = active_model_query.single() {
        commands.entity(active_model.0).despawn();
        active_model.1.next()
    } else {
        Model::Robot
    };

    let gateway_core_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("https://media.githubusercontent.com/media/Synphonyte/leptos-bevy-canvas/main/examples/loading_screen/assets/models/{}", next_model.src())));

    assets_loading.add(&gateway_core_handle);

    // Gateway Core from asset_server
    commands.spawn((
        SceneRoot(gateway_core_handle),
        next_model.initial_transform(),
        next_model,
    ));
}
