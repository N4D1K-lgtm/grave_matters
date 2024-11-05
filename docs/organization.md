# Project Organization

## Introduction

Scratch pad for my brain. This will be quick links, file structure, project design decisions etc.

## Directory Structure

### Root

```bash
grave_matters/
├── Cargo.toml
├── assets/ # all assets
├── docs/ # all documentation related to the project
├── web/ # basic files needed for the wasm build.
├── flake.nix # this contains the dev shell needed to compile and run bevy
├── Trunk.toml # trunk configuration for wasm build
└── src/
```

### Source Code `src/`

```bash
src/
├── main.rs # entrypoint. Adds default plugins and `GamePlugin`
├── app.rs # contains `GamePlugin` and initializes global state.
├── globals/ # any code that is shared globally and **must** be initialized in a single place
└── plugins/ # every feature should be organized in a plugin. If it can be a plugin its a plugin
```

### Plugin Structure

Plugins should generally follow this structure

```bash
plugins/
└── my_plugin/
    ├── nested_subplugin/ # Optional nested plugin. ie. ui, audio
    ├── components/       # Bevy components specific to this plugin.
    ├── resources/        # Bevy resources used by the plugin.
    ├── systems/          # Bevy systems defining behavior.
    ├── states/           # Scoped states for this plugin.
    ├── dev/              # Code conditionally compiled out in release builds.
    └── mod.rs            # Module file for the plugin.
```

Here is a specific example

```bash
plugins/
└── inventory_plugin/
    ├── components/
    │   ├── item.rs
    │   └── inventory.rs
    ├── resources/
    │   └── inventory_config.rs
    ├── systems/
    │   ├── item_pickup_system.rs
    │   └── inventory_management_system.rs
    ├── states/
    │   └── inventory_state.rs
    ├── dev/
    │   └── debug_inventory.rs
    └── mod.rs
```
