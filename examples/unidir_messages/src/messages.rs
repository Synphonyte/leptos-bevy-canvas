use bevy::prelude::*;

#[derive(Message, Debug)]
pub struct TextMessage {
    pub text: String,
}

#[derive(Message, Debug, Copy, Clone)]
pub struct ClickMessage {
    pub char_index: usize,
}
