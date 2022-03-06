use bevy::prelude::*;

use crate::GameState;

use super::{
    get_first_obstacle_pos_downward, to_tile_space, ObstaclesRes, Player, Point,
    PLAYER_BLINK_DURATION, TILE_SIZE,
};

const GOO_INITIAL_POS: f32 = -50.0;
const GOO_SPEED: f32 = 32.0;
const GOO_SIN_AMPLITUDE: f32 = 8.0;
const GOO_HIT_REGRESS: f32 = 64.0;

#[derive(Component)]
pub struct Goo {
    y: f32,
    start_time: f64,
    regress: f32,
}

impl Goo {
    pub fn new(start_time: f64) -> Goo {
        Goo {
            y: GOO_INITIAL_POS,
            start_time,
            regress: 0.0,
        }
    }
}

pub fn goo_movement(
    time: Res<Time>,
    players: Query<&Transform, (With<Player>, Without<Goo>)>,
    mut goo_query: Query<(&mut Goo, &mut Transform, &Sprite)>,
) {
    let player_transform = players.single();
    let (mut goo, mut transform, sprite) = goo_query.single_mut();

    goo.y = GOO_INITIAL_POS - goo.regress
        + (time.seconds_since_startup() - goo.start_time) as f32 * GOO_SPEED
        + ((time.seconds_since_startup() * 2.0).sin() as f32) * GOO_SIN_AMPLITUDE;

    transform.translation.x = player_transform.translation.x;
    transform.translation.y = goo.y - sprite.custom_size.unwrap().y / 2.0;
    transform.translation.z = 500.0;
}

pub fn goo_collision(
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
    mut player_query: Query<(&Transform, &mut Player), (With<Player>, Without<Goo>)>,
    mut app_state: ResMut<State<GameState>>,
    mut goo_query: Query<&mut Goo>,
) {
    let mut goo = goo_query.single_mut();
    let (player_transform, mut player) = player_query.single_mut();

    if player_transform.translation.y < goo.y {
        player.lifes -= 1;

        if player.lifes == 0 {
            app_state.set(GameState::LoseMenu).unwrap();
        } else {
            let first_down_obstacle_tile_pos = get_first_obstacle_pos_downward(
                &obstacles.map,
                to_tile_space(Point(
                    player_transform.translation.x,
                    player_transform.translation.y,
                )),
            )
            .unwrap();

            let floor_y = first_down_obstacle_tile_pos.1 as f32 * TILE_SIZE + TILE_SIZE;
            let distance_to_floor = goo.y - floor_y;

            player.blink_until = time.seconds_since_startup() + PLAYER_BLINK_DURATION;
            goo.regress += distance_to_floor + GOO_HIT_REGRESS;
        }
    }
}