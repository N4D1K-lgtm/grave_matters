use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::prelude::{AppState, UpdateSet};
use systems::*;

mod components;
mod resources;
mod systems;

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR))
        .add_systems(OnEnter(AppState::Splash), setup_splash_screen)
        .add_systems(
            Update,
            (
                tick_fade_in_out.in_set(UpdateSet::TickTimers),
                apply_fade_in_out.in_set(UpdateSet::Update),
            ),
        );

    app.register_type::<resources::SplashTimer>();

    app.add_systems(OnEnter(AppState::Splash), insert_splash_timer);
    app.add_systems(OnExit(AppState::Splash), remove_splash_timer);

    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(UpdateSet::TickTimers),
            check_splash_timer.in_set(UpdateSet::Update),
        )
            .run_if(in_state(AppState::Splash)),
    );

    app.add_systems(
        Update,
        continue_to_loading_screen
            .run_if(input_just_pressed(KeyCode::Escape).and_then(in_state(AppState::Splash))),
    );
}
