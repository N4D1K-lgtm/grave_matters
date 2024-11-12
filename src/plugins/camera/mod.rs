mod components;
mod systems;

use crate::prelude::{AppState, GameState};

pub use components::*;
use systems::*;

use bevy::prelude::*;
use bevy_dolly::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);

    app.add_systems(Update, Dolly::<MainCamera>::update_active);
}
