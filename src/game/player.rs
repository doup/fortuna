use benimator::SpriteSheetAnimation;
use bevy::prelude::*;

use crate::stats::StatsRes;

use super::{
    events::{CeilHitEvent, LandingEvent},
    obstacles::{get_obstacle_list, get_tile_list, get_tile_space_bbox, BBox},
    Animations, ObstaclesRes, Player, PlayerDirection, Position, Velocity, GRAVITY, PLAYER_HEIGHT,
    PLAYER_WIDTH, TILE_SIZE,
};

const SKIN_SIZE: f32 = 2.0;
const PLAYER_SPRITE_HEIGHT: f32 = 48.0;
const PLAYER_WIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
const PLAYER_HEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;

pub fn player_color(
    _stats: Res<StatsRes>,
    mut _player_query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    // let mut sprite = player_query.single_mut();

    // sprite.color = match stats.value.color {
    //     SkinColor::Light => Color::hex("b8ddf5").unwrap(),
    //     SkinColor::Medium => Color::hex("3f789d").unwrap(),
    //     SkinColor::Dark => Color::hex("103954").unwrap(),
    // };
}

pub fn player_movement(
    stats: Res<StatsRes>,
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
    mut landing_event: EventWriter<LandingEvent>,
    mut ceil_hit_event: EventWriter<CeilHitEvent>,
    mut player_query: Query<(&mut Position, &mut Velocity, &mut Player), With<Player>>,
) {
    let (mut position, mut velocity, mut player) = player_query.single_mut();
    let time_delta = time.delta_seconds();

    velocity.y += GRAVITY * time_delta;

    let is_moving_right = velocity.x > 0.0;
    let is_moving_up = velocity.y > 0.0;

    if velocity.x != 0.0 {
        let pos_x = position.value.x + velocity.x * time_delta;
        let bottom = position.value.y - PLAYER_HEIGHT_HALF + SKIN_SIZE;
        let top = position.value.y + PLAYER_HEIGHT_HALF - SKIN_SIZE;

        let horizontal_bbox = if is_moving_right {
            let left = position.value.x + PLAYER_WIDTH_HALF;
            let right = left + velocity.x.abs() * time_delta;
            BBox::new(left, bottom, right, top)
        } else {
            let right = position.value.x - PLAYER_WIDTH_HALF;
            let left = right - velocity.x.abs() * time_delta;
            BBox::new(left, bottom, right, top)
        };

        let horizontal_obstacles = get_obstacle_list(
            get_tile_list(get_tile_space_bbox(&horizontal_bbox)),
            &obstacles.map,
            true,
        );

        let nearest_obstacle_x = if is_moving_right {
            let tile_x = horizontal_obstacles.iter().map(|o| o.pos.0).min();
            tile_x.map(|x| x as f32 * TILE_SIZE - PLAYER_WIDTH_HALF)
        } else {
            let tile_x = horizontal_obstacles.iter().map(|o| o.pos.0).max();
            tile_x.map(|x| x as f32 * TILE_SIZE + TILE_SIZE + PLAYER_WIDTH_HALF)
        };

        if nearest_obstacle_x.is_some() {
            let nearest_obstacle_x = nearest_obstacle_x.unwrap();

            position.value.x = if is_moving_right {
                pos_x.min(nearest_obstacle_x)
            } else {
                pos_x.max(nearest_obstacle_x)
            };
        } else {
            position.value.x = pos_x;
        }
    }

    if velocity.y != 0.0 {
        let pos_y = position.value.y + velocity.y * time_delta;
        let left = position.value.x - PLAYER_WIDTH_HALF + SKIN_SIZE;
        let right = position.value.x + PLAYER_WIDTH_HALF - SKIN_SIZE;

        let vertical_bbox = if is_moving_up {
            let bottom = position.value.y + PLAYER_HEIGHT_HALF;
            let top = bottom + velocity.y.abs() * time_delta;
            BBox::new(left, bottom, right, top)
        } else {
            let top = position.value.y - PLAYER_HEIGHT_HALF;
            let bottom = top - velocity.y.abs() * time_delta;
            BBox::new(left, bottom, right, top)
        };

        let vertical_obstacles = get_obstacle_list(
            get_tile_list(get_tile_space_bbox(&vertical_bbox)),
            &obstacles.map,
            is_moving_up && stats.value.can_skip_one_way_platforms,
        );

        let nearest_obstacle_y = if is_moving_up {
            let tile_y = vertical_obstacles.iter().map(|o| o.pos.1).min();
            tile_y.map(|y| y as f32 * TILE_SIZE - PLAYER_HEIGHT_HALF)
        } else {
            let tile_y = vertical_obstacles.iter().map(|o| o.pos.1).max();
            tile_y.map(|y| y as f32 * TILE_SIZE + TILE_SIZE + PLAYER_HEIGHT_HALF)
        };

        if nearest_obstacle_y.is_some() {
            let nearest_obstacle_y = nearest_obstacle_y.unwrap();

            if is_moving_up && (nearest_obstacle_y < pos_y)
                || !is_moving_up && (nearest_obstacle_y > pos_y)
            {
                position.value.y = nearest_obstacle_y;

                if !is_moving_up {
                    if player.last_ground_time.is_none() {
                        landing_event.send(LandingEvent {
                            position: position.value.clone(),
                            velocity: Vec2::new(velocity.x, velocity.y),
                        });
                    }

                    velocity.y = 0.0;
                    player.last_ground_time = Some(time.seconds_since_startup());
                } else {
                    velocity.y = -velocity.y * 0.1;
                    ceil_hit_event.send(CeilHitEvent {
                        position: position.value.clone(),
                    });
                }
            } else {
                position.value.y = pos_y;
            };
        } else {
            position.value.y = pos_y;
        }
    }
}

pub fn player_animation(
    animations: Res<Animations>,
    stats: Res<StatsRes>,
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut Handle<SpriteSheetAnimation>,
            &Player,
            &Position,
            &Velocity,
        ),
        With<Player>,
    >,
) {
    let (mut sprite_transform, mut sprite, mut animation, player, position, velocity) =
        player_query.single_mut();

    let is_grounded = velocity.y == 0.0;
    let is_running = is_grounded && velocity.x != 0.0;
    let is_jumping = !is_grounded;
    let is_buffered_jump_valid = player.is_buffered_jump_valid(time.seconds_since_startup());

    sprite_transform.translation.x = position.value.x;
    sprite_transform.translation.y =
        position.value.y + (PLAYER_SPRITE_HEIGHT - PLAYER_HEIGHT) / 2.0;
    sprite_transform.translation.z = 10.0;

    sprite.flip_x = player.direction == PlayerDirection::Left;

    if is_jumping {
        // Map velocity.y to animation frame
        let force_range = stats.value.jump_force * 2.0;
        let total_frames = animations.jump.len() as f32;
        let velocity = velocity
            .y
            .clamp(-stats.value.jump_force, stats.value.jump_force);
        let frame = (force_range - (velocity + stats.value.jump_force)) / force_range;
        let frame = (frame * total_frames).min(total_frames - 1.0) as usize;

        *animation = animations.jump[frame].clone();
    } else if is_buffered_jump_valid {
        *animation = animations.jump[0].clone();
    } else if is_running {
        *animation = animations.run.clone();
    } else {
        *animation = animations.idle.clone();
    }
}
