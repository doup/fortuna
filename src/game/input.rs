use bevy::prelude::*;

use crate::{stats::StatsRes, GameState};

use super::{
    bouncer,
    events::{DirectionChangeEvent, JumpEvent},
    Player, PlayerDirection, Position, Velocity, GRAVITY,
};

const COYOTE_TIME: f64 = 0.125; // seconds after falling from a platform that still can jump

pub fn handle_input(
    stats: Res<StatsRes>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut jump_event: EventWriter<JumpEvent>,
    mut direction_change_event: EventWriter<DirectionChangeEvent>,
    mut app_state: ResMut<State<GameState>>,
    mut player_query: Query<(&mut Velocity, &mut Player, &Position), With<Player>>,
) {
    let (mut velocity, mut player, position) = player_query.single_mut();
    let time_delta = time.delta_seconds();
    let top_speed;
    let top_speed_rate;
    let stop_rate;
    let jump_force;

    if player.depressed_until > time.seconds_since_startup() {
        top_speed = stats.value.top_speed_depressed;
        top_speed_rate = stats.value.top_speed_rate_depressed;
        stop_rate = stats.value.stop_rate_depressed;
        jump_force = stats.value.jump_force_depressed;
    } else {
        top_speed = stats.value.top_speed;
        top_speed_rate = stats.value.top_speed_rate;
        stop_rate = stats.value.stop_rate;
        jump_force = stats.value.jump_force;
    }

    if let Some(last_ground_time) = player.last_ground_time {
        // Player is in the ground
        let jump_force = jump_force - GRAVITY * time_delta; // Correct by gravity… why?? I don't remember :facepalm:
        let is_in_jump_window = time.seconds_since_startup() < last_ground_time + COYOTE_TIME;
        let is_buffered_jump_valid = player.is_buffered_jump_valid(time.seconds_since_startup());
        let can_jump =
            keys.just_pressed(KeyCode::Space) && is_in_jump_window || is_buffered_jump_valid;

        if !is_in_jump_window {
            player.last_ground_time = None;
        }

        if can_jump {
            let is_grounded = velocity.y >= 0.0;

            velocity.y = jump_force;
            player.last_ground_time = None;

            jump_event.send(JumpEvent {
                is_grounded,
                position: position.value.clone(),
                velocity: Vec2::new(velocity.x, velocity.y),
            });
        }

        // Clear buffered jump time
        player.buffer_jump_time = None;
    } else if keys.just_pressed(KeyCode::Space) {
        // Player is in the air AND "space" is pressed
        player.buffer_jump_time = Some(time.seconds_since_startup());
    }

    if keys.just_pressed(KeyCode::R) {
        app_state.set(GameState::MainMenu).unwrap();
    }

    if keys.pressed(KeyCode::Left) {
        if player.direction != PlayerDirection::Left {
            direction_change_event.send(DirectionChangeEvent {
                position: position.value.clone(),
                new_direction: PlayerDirection::Left,
            });
        }

        player.direction = PlayerDirection::Left;
        velocity.x = (velocity.x - top_speed_rate * time_delta).max(-top_speed);
    } else if keys.pressed(KeyCode::Right) {
        if player.direction != PlayerDirection::Right {
            direction_change_event.send(DirectionChangeEvent {
                position: position.value.clone(),
                new_direction: PlayerDirection::Right,
            });
        }

        player.direction = PlayerDirection::Right;
        velocity.x = (velocity.x + top_speed_rate * time_delta).min(top_speed);
    } else if velocity.x > 0.0 {
        velocity.x = (velocity.x - stop_rate * time_delta).max(0.0);
    } else if velocity.x < 0.0 {
        velocity.x = (velocity.x + stop_rate * time_delta).min(0.0);
    }

    // Apply bounce force
    if let Some(mut bounce_force) = player.bounce_force {
        velocity.x += bounce_force * time_delta;

        bounce_force = if bounce_force > 0.0 {
            (bounce_force - (bouncer::BOUNCER_FORCE / bouncer::BOUNCER_DURATION) * time_delta)
                .max(0.0)
        } else if bounce_force < 0.0 {
            (bounce_force + (bouncer::BOUNCER_FORCE / bouncer::BOUNCER_DURATION) * time_delta)
                .min(0.0)
        } else {
            0.0
        };

        if bounce_force == 0.0 {
            player.bounce_force = None;
        } else {
            player.bounce_force = Some(bounce_force);
        }
    }
}
