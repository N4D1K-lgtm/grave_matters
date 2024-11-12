use super::{
    components::UiImageFadeInOut, resources::SplashTimer, SPLASH_BACKGROUND_COLOR,
    SPLASH_DURATION_SECS, SPLASH_FADE_DURATION_SECS,
};
use crate::prelude::{AppState, Containers};
use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

pub(super) fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((
            Name::new("Splash Background"),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(AppState::Splash),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Splash Logo"),
                ImageBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        width: Val::Percent(70.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load_with_settings(
                        // This should be an embedded asset for instant loading, but that is
                        // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                        "images/splash.png",
                        |settings: &mut ImageLoaderSettings| {
                            // Make an exception for the splash image in case
                            // `ImagePlugin::default_nearest()` is used for pixel art.
                            settings.sampler = ImageSampler::linear();
                        },
                    )),
                    ..default()
                },
                UiImageFadeInOut {
                    total_duration: SPLASH_DURATION_SECS,
                    fade_duration: SPLASH_FADE_DURATION_SECS,
                    t: 0.0,
                },
            ));
        });
}

pub(super) fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut UiImageFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_seconds();
    }
}

pub(super) fn apply_fade_in_out(mut animation_query: Query<(&UiImageFadeInOut, &mut UiImage)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

pub(super) fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

pub(super) fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

pub(super) fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

pub(super) fn check_splash_timer(
    timer: ResMut<SplashTimer>,
    mut next_screen: ResMut<NextState<AppState>>,
) {
    if timer.0.just_finished() {
        next_screen.set(AppState::Loading);
    }
}
pub(super) fn continue_to_loading_screen(mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Loading);
}
