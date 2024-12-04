use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct CharIndex(usize);

impl CharIndex {
    pub fn new(index: usize) -> Self {
        CharIndex(index)
    }
}