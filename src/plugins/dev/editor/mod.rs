use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EditorPlugin::default());
}
