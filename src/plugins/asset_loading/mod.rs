use crate::prelude::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::Title));
}
