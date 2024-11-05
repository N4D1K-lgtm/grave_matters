// app construction and initialization
mod app;
// almost all of the app logic
mod plugins;
// constructs that DON'T belong in a plugin or group of plugins. Ie. `AppState` will be reused by
// quite a few plugins
pub mod global;
// convienient reexports
pub mod prelude;
