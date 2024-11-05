use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AtmospherePlugin);
}
