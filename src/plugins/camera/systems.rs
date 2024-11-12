use super::{MainCamera, Rotates, UiCamera};
use crate::prelude::{Layer, RenderLayerExt};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_dolly::{drivers::follow::MovableLookAt, prelude::*};

use std::f32::consts::PI;

pub(super) fn setup_camera(mut commands: Commands) {
    let start_pos = Vec3::new(0., 0., 0.);

    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Rig::builder()
            .with(MovableLookAt::from_position_target(start_pos))
            .build(),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
    ));
}

pub(super) fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

pub(super) fn update_camera(q0: Query<&Transform, With<Rotates>>, mut q1: Query<&mut Rig>) {
    let player = q0.single().to_owned();
    let mut rig = q1.single_mut();

    rig.driver_mut::<MovableLookAt>()
        .set_position_target(player.translation, player.rotation);
}
