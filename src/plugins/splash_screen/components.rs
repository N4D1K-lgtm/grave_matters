use super::{SPLASH_DURATION_SECS, SPLASH_FADE_DURATION_SECS};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct UiImageFadeInOut {
    /// Total duration in seconds.
    pub total_duration: f32,
    /// Fade duration in seconds.
    pub fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    pub t: f32,
}

impl UiImageFadeInOut {
    pub fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

impl Default for UiImageFadeInOut {
    fn default() -> Self {
        Self {
            total_duration: SPLASH_DURATION_SECS,
            fade_duration: SPLASH_FADE_DURATION_SECS,
            ..default()
        }
    }
}
