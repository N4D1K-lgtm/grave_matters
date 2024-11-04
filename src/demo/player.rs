//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use leafwing_input_manager::prelude::*;

use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use bevy_asset_loader::prelude::*;

use crate::{
    demo::{
        animation::PlayerAnimation,
        movement::{Action, MovementController, ScreenWrap},
    },
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.configure_loading_state(
        LoadingStateConfig::new(Screen::Loading).load_collection::<PlayerAssets>(),
    );

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

impl Player {
    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();
        input_map.insert(Action::Up, KeyCode::KeyW);
        input_map.insert(Action::Up, KeyCode::ArrowUp);
        input_map.insert(Action::Down, KeyCode::KeyS);
        input_map.insert(Action::Down, KeyCode::ArrowDown);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Left, KeyCode::ArrowLeft);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Right, KeyCode::ArrowRight);
        input_map
    }
}
/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    /// See [`MovementController::max_speed`].
    pub max_speed: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple
    // sprites. By attaching it to a [`SpriteBundle`] and providing an index, we
    // can specify which section of the image we want to see. We will use this
    // to animate our player character. You can learn more about texture atlases in
    // this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: player_assets.ducky.clone(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController {
            max_speed: config.max_speed,
            ..default()
        },
        ScreenWrap,
        player_animation,
        StateScoped(Screen::Gameplay),
        InputManagerBundle {
            input_map: Player::default_input_map(),
            ..default()
        },
    ));
}

fn record_player_directional_input(
    mut query: Query<(&ActionState<Action>, &mut MovementController), With<Player>>,
) {
    for (action_state, mut controller) in &mut query {
        let mut intent = Vec2::ZERO;

        if action_state.pressed(&Action::Up) {
            intent.y += 1.0;
        }
        if action_state.pressed(&Action::Down) {
            intent.y -= 1.0;
        }
        if action_state.pressed(&Action::Left) {
            intent.x -= 1.0;
        }
        if action_state.pressed(&Action::Right) {
            intent.x += 1.0;
        }

        controller.intent = intent.normalize_or_zero();
    }
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler = nearest))]
    pub ducky: Handle<Image>,
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg"
        ),
        collection(typed)
    )]
    pub steps: Vec<Handle<AudioSource>>,
}
