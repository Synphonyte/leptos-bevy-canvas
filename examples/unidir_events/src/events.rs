use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct TextEvent {
    pub text: String,
}

#[derive(Event, Debug)]
pub struct ClickEvent {
    pub char_index: usize,
}
