use bevy::prelude::*;

#[cfg(feature = "dev")]
mod dev;

mod atmosphere;
mod camera;
mod input;
mod voxel;

pub fn plugins(app: &mut App) {
    app.add_plugins((voxel::plugin, atmosphere::plugin, input::plugin));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}
