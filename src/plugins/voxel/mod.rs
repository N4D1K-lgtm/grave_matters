use bevy::prelude::*;
use bevy_vox_scene::VoxScenePlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VoxScenePlugin::default());
}
