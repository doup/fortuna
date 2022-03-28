use bevy::prelude::*;

use crate::GameState;

use super::{
    camera::GameCamera,
    obstacles::{get_first_obstacle_pos_downward, to_tile_space},
    GameStateEntity, ObstaclesRes, Player, Position, TILE_SIZE,
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

pub fn setup_goo(mut commands: Commands, time: Res<Time>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Goo::new(time.seconds_since_startup()))
        .insert(GameStateEntity);
}

pub fn goo_movement(
    time: Res<Time>,
    cameras: Query<&Transform, (With<GameCamera>, Without<Goo>)>,
    mut goo_query: Query<(&mut Goo, &mut Transform, &Sprite)>,
) {
    let camera_position = cameras.single();
    let (mut goo, mut transform, sprite) = goo_query.single_mut();

    // goo.y = GOO_INITIAL_POS;
    goo.y = GOO_INITIAL_POS - goo.regress
        + (time.seconds_since_startup() - goo.start_time) as f32 * GOO_SPEED
        + ((time.seconds_since_startup() * 2.0).sin() as f32) * GOO_SIN_AMPLITUDE;

    transform.translation.x = camera_position.translation.x;
    transform.translation.y = goo.y - sprite.custom_size.unwrap().y / 2.0;
    transform.translation.z = 500.0;
}

pub fn goo_collision(
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
    mut player_query: Query<(&Position, &mut Player), (With<Player>, Without<Goo>)>,
    mut app_state: ResMut<State<GameState>>,
    mut goo_query: Query<&mut Goo>,
) {
    let mut goo = goo_query.single_mut();
    let (player_position, mut player) = player_query.single_mut();

    if player_position.value.y < goo.y {
        player.lifes -= 1;

        if player.lifes == 0 {
            app_state.set(GameState::LoseMenu).unwrap();
        } else {
            let first_down_obstacle_tile_pos = get_first_obstacle_pos_downward(
                &obstacles.map,
                to_tile_space(&player_position.value),
            )
            .unwrap();

            let floor_y = first_down_obstacle_tile_pos.1 as f32 * TILE_SIZE + TILE_SIZE;
            let distance_to_floor = goo.y - floor_y;

            player.blink(time.seconds_since_startup(), None);
            goo.regress += distance_to_floor + GOO_HIT_REGRESS;
        }
    }
}
