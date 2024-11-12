mod components;
mod systems;

pub use components::MainCamera;

use bevy::prelude::*;
use bevy_dolly::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, Dolly::<MainCamera>::update_active);
}
