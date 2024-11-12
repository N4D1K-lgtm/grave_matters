use bevy::prelude::*;

#[cfg(feature = "dev")]
mod dev;

mod asset_loading;
mod atmosphere;
mod camera;
mod input;
mod splash_screen;
mod voxel;

pub use camera::MainCamera;

pub fn plugins(app: &mut App) {
    app.add_plugins((
        voxel::plugin,
        atmosphere::plugin,
        input::plugin,
        camera::plugin,
        splash_screen::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}
