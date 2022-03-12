use crate::GameState;

use super::{Player, Position};
use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component)]
pub struct Goal;

pub fn goal_collision(
    mut app_state: ResMut<State<GameState>>,
    mut player_query: Query<(&Position, &Sprite), (With<Player>, Without<Goal>)>,
    goal_query: Query<(&Transform, &Sprite), (With<Goal>, Without<Player>)>,
) {
    let (player_position, player_sprite) = player_query.single_mut();
    let (goal_transform, goal_sprite) = goal_query.single();

    if collide(
        player_position.value,
        player_sprite.custom_size.unwrap(),
        goal_transform.translation,
        goal_sprite.custom_size.unwrap(),
    )
    .is_some()
    {
        app_state.set(GameState::WinMenu).unwrap();
    }
}
