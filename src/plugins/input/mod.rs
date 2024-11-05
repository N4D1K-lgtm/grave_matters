use crate::prelude::*;
use bevy::prelude::App;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
}
