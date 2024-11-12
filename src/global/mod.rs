use crate::prelude::{AppState, UpdateSet};
use bevy::prelude::*;

pub mod actions;
pub mod components;
pub mod events;
pub mod layers;
pub mod resources;
pub mod sets;
pub mod states;
pub mod systems;

pub fn global_plugin(app: &mut App) {
    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .configure_sets(
            Update,
            (
                UpdateSet::TickTimers,
                UpdateSet::RecordInput,
                UpdateSet::Update,
            )
                .chain(),
        );
}
