use super::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn camera_movement(
    players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, With<GameCamera>>,
) {
    let player_transform = players.single();
    let mut camera_transform = cameras.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    // camera_transform.translation.x = 208.0;
    // camera_transform.translation.y = 192.0;
    // println!("{:?}", camera_transform.translation);
}
