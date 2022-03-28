use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;

use super::{
    events::{CeilHitEvent, DirectionChangeEvent, JumpEvent, LandingEvent},
    Animations, Player,
};

#[derive(Component)]
pub struct Vfx;

#[derive(PartialEq)]
enum AlignVfx {
    Top,
    Bottom,
}

fn add_vfx(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    animation: Handle<SpriteSheetAnimation>,
    position: Vec2,
    align: AlignVfx,
) {
    let position = if align == AlignVfx::Top {
        position + Vec2::new(0.0, 16.0)
    } else {
        position - Vec2::new(0.0, 16.0)
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: position.extend(10.0),
                ..Default::default()
            },
            texture_atlas,
            ..Default::default()
        })
        .insert(animation)
        .insert(Play)
        .insert(Vfx);
}

pub fn blink_player(time: Res<Time>, mut player_query: Query<(&Player, &mut Visibility)>) {
    let (player, mut visibility) = player_query.single_mut();
    let time_seconds = time.seconds_since_startup();

    if player.blink_until > time_seconds {
        visibility.is_visible = (time_seconds * 10.0) as i32 % 2 == 0;
    } else {
        visibility.is_visible = true;
    }
}

pub fn remove_vfx(dust_query: Query<(Entity, Option<&Play>), With<Vfx>>, mut commands: Commands) {
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
        add_vfx(
            &mut commands,
            animations.vfx_atlas.clone(),
            animations.run_jump_dust.clone(),
            ev.position,
            AlignVfx::Bottom,
        );
    }
}

pub fn add_landing_dust(
    animations: Res<Animations>,
    mut landing_event: EventReader<LandingEvent>,
    mut commands: Commands,
) {
    for ev in landing_event.iter() {
        add_vfx(
            &mut commands,
            animations.vfx_atlas.clone(),
            animations.vfx_debug.clone(),
            ev.position,
            AlignVfx::Bottom,
        );
    }
}

pub fn add_direction_change_dust(
    animations: Res<Animations>,
    mut landing_event: EventReader<DirectionChangeEvent>,
    mut commands: Commands,
) {
    for ev in landing_event.iter() {
        add_vfx(
            &mut commands,
            animations.vfx_atlas.clone(),
            animations.vfx_debug.clone(),
            ev.position,
            AlignVfx::Bottom,
        );
    }
}

pub fn add_ceil_hit_sprite(
    animations: Res<Animations>,
    mut ceil_hit_event: EventReader<CeilHitEvent>,
    mut commands: Commands,
) {
    for ev in ceil_hit_event.iter() {
        add_vfx(
            &mut commands,
            animations.vfx_atlas.clone(),
            animations.vfx_debug.clone(),
            ev.position,
            AlignVfx::Top,
        );
    }
}
