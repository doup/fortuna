use benimator::Play;
use bevy::prelude::*;

use super::{
    events::{CeilHitEvent, DirectionChangeEvent, JumpEvent, LandingEvent},
    Animations, Player,
};

#[derive(Component)]
pub struct VFX;

pub fn blink_player(time: Res<Time>, mut player_query: Query<(&Player, &mut Visibility)>) {
    let (player, mut visibility) = player_query.single_mut();
    let time_seconds = time.seconds_since_startup();

    if player.blink_until > time_seconds {
        visibility.is_visible = (time_seconds * 10.0) as i32 % 2 == 0;
    } else {
        visibility.is_visible = true;
    }
}

pub fn remove_vfx(dust_query: Query<(Entity, Option<&Play>), With<VFX>>, mut commands: Commands) {
    for (entity, maybe_play) in dust_query.iter() {
        if maybe_play.is_none() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn add_jump_dust(
    animations: Res<Animations>,
    mut jump_event: EventReader<JumpEvent>,
    mut commands: Commands,
) {
    for ev in jump_event.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: (ev.position + Vec2::new(0.0, -8.0)).extend(10.0),
                    ..Default::default()
                },
                texture_atlas: animations.dust_atlas.clone(),
                ..Default::default()
            })
            .insert(animations.jump_dust.clone())
            .insert(Play)
            .insert(VFX);
    }
}

pub fn add_landing_dust(
    animations: Res<Animations>,
    mut landing_event: EventReader<LandingEvent>,
    mut commands: Commands,
) {
    for ev in landing_event.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: (ev.position + Vec2::new(0.0, -8.0)).extend(10.0),
                    ..Default::default()
                },
                texture_atlas: animations.dust_atlas.clone(),
                ..Default::default()
            })
            .insert(animations.landing_dust.clone())
            .insert(Play)
            .insert(VFX);
    }
}

pub fn add_direction_change_dust(
    animations: Res<Animations>,
    mut landing_event: EventReader<DirectionChangeEvent>,
    mut commands: Commands,
) {
    for ev in landing_event.iter() {
        println!("{:?}", ev);
        // commands
        //     .spawn_bundle(SpriteSheetBundle {
        //         transform: Transform {
        //             translation: (ev.position + Vec2::new(0.0, -8.0)).extend(10.0),
        //             ..Default::default()
        //         },
        //         texture_atlas: animations.dust_atlas.clone(),
        //         ..Default::default()
        //     })
        //     .insert(animations.landing_dust.clone())
        //     .insert(Play)
        //     .insert(VFX);
    }
}

pub fn add_ceil_hit_sprite(mut ceil_hit_event: EventReader<CeilHitEvent>) {
    for ev in ceil_hit_event.iter() {
        // ev.
    }
}
