use bevy::prelude::*;

use super::PlayerDirection;

#[derive(Debug)]
pub struct JumpEvent {
    pub is_grounded: bool,
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct LandingEvent {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct CeilHitEvent {
    pub position: Vec2,
}

#[derive(Debug)]
pub struct DirectionChangeEvent {
    pub position: Vec2,
    pub new_direction: PlayerDirection,
}
