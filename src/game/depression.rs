use bevy::prelude::*;
use rand::Rng;

use crate::stats::StatsRes;

use super::{DepressedText, Player};

// DEPRESSIVE STATE
pub const MIN_DEPRE_DURATION: f64 = 2.0;
pub const MAX_DEPRE_DURATION: f64 = 6.0;
pub const MIN_TIME_BETWEEN_DEPRE: f64 = 10.0;

pub fn show_depressed_text(
    time: Res<Time>,
    mut depressed_text_query: Query<
        (&mut Visibility, &mut Transform),
        (With<DepressedText>, Without<Player>),
    >,
    player_query: Query<(&Player, &Transform), With<Player>>,
) {
    let (player, transform) = player_query.single();
    let (mut depre_visibility, mut depre_transform) = depressed_text_query.single_mut();

    depre_visibility.is_visible = player.depressed_until > time.seconds_since_startup();
    depre_transform.translation = transform.translation.clone() + Vec3::new(0.0, 24.0, 0.0);
}

pub fn trigger_depression(stats: Res<StatsRes>, time: Res<Time>, mut players: Query<&mut Player>) {
    let mut player = players.single_mut();

    let can_get_depressed = stats.value.is_depressive
        && (player.depressed_until + MIN_TIME_BETWEEN_DEPRE) < time.seconds_since_startup();

    if can_get_depressed {
        if rand::thread_rng().gen_range(0.0..1.0) < stats.value.depre_chance {
            player.depressed_until = time.seconds_since_startup()
                + rand::thread_rng().gen_range(MIN_DEPRE_DURATION..MAX_DEPRE_DURATION);
        }
    }
}
