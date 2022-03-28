use bevy::prelude::*;

use super::events::{CeilHitEvent, JumpEvent, LandingEvent};
use crate::loading::GameAssets;

pub fn play_jump_sound(
    audio: Res<Audio>,
    game_assets: Res<GameAssets>,
    mut jump_event: EventReader<JumpEvent>,
) {
    for _ in jump_event.iter() {
        audio.play(game_assets.jump_sound.clone());
    }
}

pub fn play_landing_sound(mut landing_event: EventReader<LandingEvent>) {
    for ev in landing_event.iter() {
        println!("Play landing sound");
    }
}

pub fn play_ceil_hit_sound(mut ceil_hit_event: EventReader<CeilHitEvent>) {
    for ev in ceil_hit_event.iter() {
        println!("Play ceil-hit sound");
    }
}
