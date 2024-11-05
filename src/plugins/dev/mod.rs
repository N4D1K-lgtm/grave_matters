mod editor;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(editor::plugin);
}
