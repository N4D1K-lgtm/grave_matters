use super::MainCamera;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera, // The rig component tag
        Rig::builder()
            .with(Position::new(Vec3::ZERO)) // Start position
            // Adds a driver with the method rotate_yaw_pitch
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            // Interpolation when the translation is updated, also known as smoothing
            .with(Smooth::new_position(0.3))
            // Interpolation when the rotation is updated (updated via the YawPitch driver)
            .with(Smooth::new_rotation(0.3))
            // Moves the camera point out in the Z direction and uses the position as the pivot
            .with(Arm::new(Vec3::Z * 4.0))
            .build(),
        Camera3dBundle::default(),
    ));
}


fn
