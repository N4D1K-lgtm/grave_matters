//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
    app.add_loading_state(LoadingState::new(Screen::Loading).continue_to_state(Screen::Title));

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
}
