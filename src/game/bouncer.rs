use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_ldtk::prelude::*;

use crate::stats::{SkinColor, StatsRes, Wealth};

use super::{Player, Position, PLAYER_HEIGHT, PLAYER_WIDTH};

pub const BOUNCER_FORCE: f32 = 2500.0;
pub const BOUNCER_DURATION: f32 = 0.5;

#[derive(Debug)]
enum BouncerType {
    WealthRich,
    SkinColorLight,
}

#[derive(Debug, Component)]
pub struct Bouncer {
    allow: BouncerType,
    direction: f32,
}

pub fn get_bouncer_from_entity_instance(entity: &EntityInstance) -> Bouncer {
    let bouncer_type_cfg = entity
        .field_instances
        .iter()
        .filter(|field| field.identifier == "type")
        .collect::<Vec<_>>();

    let push_left_cfg = entity
        .field_instances
        .iter()
        .filter(|field| field.identifier == "push_left")
        .collect::<Vec<_>>();

    let allow = if let FieldValue::String(bouncer_type_option) = bouncer_type_cfg[0].value.clone() {
        if let Some(bouncer_type) = bouncer_type_option {
            match bouncer_type.as_str() {
                "rich" => BouncerType::WealthRich,
                "skin_light" => BouncerType::SkinColorLight,
                _ => BouncerType::WealthRich,
            }
        } else {
            BouncerType::WealthRich
        }
    } else {
        BouncerType::WealthRich
    };

    let direction: f32 = if let FieldValue::Bool(push_left) = push_left_cfg[0].value {
        if push_left {
            -1.0
        } else {
            1.0
        }
    } else {
        -1.0
    };

    Bouncer { allow, direction }
}

pub fn bounce_player(
    stats: Res<StatsRes>,
    time: Res<Time>,
    mut player_query: Query<
        (&mut Position, &mut Player, &TextureAtlasSprite),
        (With<Player>, Without<Bouncer>),
    >,
    bouncer_query: Query<(&Transform, &Sprite, &Bouncer), (With<Bouncer>, Without<Player>)>,
) {
    let (mut player_position, mut player, player_sprite) = player_query.single_mut();

    for (bouncer_transform, bouncer_sprite, bouncer) in bouncer_query.iter() {
        let collision = collide(
            player_position.value,
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
            bouncer_transform.translation,
            bouncer_sprite.custom_size.unwrap(),
        );

        let allow = match bouncer.allow {
            BouncerType::SkinColorLight => stats.0.color == SkinColor::Light,
            BouncerType::WealthRich => stats.0.wealth == Wealth::Rich,
        };

        if !allow && collision.is_some() {
            if bouncer.direction == 1.0 {
                player_position.value.x += 2.0 * PLAYER_WIDTH;
            } else {
                player_position.value.x -= 2.0 * PLAYER_WIDTH;
            }

            if let None = player.bounce_force {
                println!("You're not allowed here: {:?}", bouncer);

                player.bounce_force = Some(BOUNCER_FORCE * bouncer.direction);
                player.blink_until = time.seconds_since_startup() + BOUNCER_DURATION as f64;
            }
        }
    }
}
