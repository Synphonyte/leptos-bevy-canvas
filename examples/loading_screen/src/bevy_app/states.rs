use bevy::prelude::*;

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    None,
    Start,
    Loading,
    Ready,
}
