use super::Player;
use bevy::prelude::*;

// const CAMERA_WINDOW_HEIGHT: f32 = 0.3; // Percent
// const CAMERA_WINDOW_WIDTH: f32 = 0.5; // Percent
// const CAMERA_SPEED: f32 = 350.0; // px/second

#[derive(Component)]
pub struct GameCamera;

pub fn camera_movement(
    timer: Res<Time>,
    players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, With<GameCamera>>,
) {
    let player_transform = players.single();
    let mut camera_transform = cameras.single_mut();

    // Smoothed position locking
    let dir = Vec2::new(
        player_transform.translation.x - camera_transform.translation.x,
        player_transform.translation.y - camera_transform.translation.y,
    );
    let length = dir.length();
    let dir = dir.normalize() * (length * timer.delta_seconds() * 3.0);

    if length > 1.0 {
        camera_transform.translation.x += dir.x;
        camera_transform.translation.y += dir.y;
    }
}

// Camera Window
// pub fn camera_window_movement(
//     window: Res<WindowDescriptor>,
//     timer: Res<Time>,
//     players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
//     mut cameras: Query<(&mut Transform, &OrthographicProjection), With<GameCamera>>,
// ) {
//     let player_transform = players.single();
//     let (mut camera_transform, camera_projection) = cameras.single_mut();

//     let player_pos = player_transform.translation;
//     let camera_pos = camera_transform.translation;
//     let width_half = (window.width * camera_projection.scale * CAMERA_WINDOW_WIDTH) / 2.0;
//     let height_half = (window.height * camera_projection.scale * CAMERA_WINDOW_HEIGHT) / 2.0;
//     let top = camera_pos.y + height_half;
//     let right = camera_pos.x + width_half;
//     let bottom = camera_pos.y - height_half;
//     let left = camera_pos.x - width_half;

//     if player_pos.x > right {
//         let diff = player_pos.x - right;
//         camera_transform.translation.x += (CAMERA_SPEED * timer.delta_seconds()).min(diff);
//     } else if player_pos.x < left {
//         let diff = left - player_pos.x;
//         camera_transform.translation.x -= (CAMERA_SPEED * timer.delta_seconds()).min(diff);
//     }

//     if player_pos.y > top {
//         let diff = player_pos.y - top;
//         camera_transform.translation.y += (CAMERA_SPEED * timer.delta_seconds()).min(diff);
//     } else if player_pos.y < bottom {
//         let diff = bottom - player_pos.y;
//         camera_transform.translation.y -= (CAMERA_SPEED * timer.delta_seconds()).min(diff);
//     }
// }
