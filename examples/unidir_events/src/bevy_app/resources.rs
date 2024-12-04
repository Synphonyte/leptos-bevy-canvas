use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct CurrentText {
    pub text: String,
    pub glyph_entities: Vec<Entity>,
}

#[derive(Resource, Clone, Copy, Default)]
pub enum SelectedGlyph {
    #[default]
    None,
    Some {
        entity: Entity,
        index: usize,
    },
}
