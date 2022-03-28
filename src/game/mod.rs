mod bouncer;
mod camera;
mod depression;
mod events;
mod goal;
mod goo;
mod input;
mod obstacles;
mod player;
mod setup;
mod sfx;
mod vfx;

use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashMap;

use crate::{
    stats::{Intelligence, MentalHealth, SkinColor, Stats, StatsRes, Strength, Wealth},
    utils::clean_state,
    GameState,
};

use self::obstacles::{Obstacle, Point};

// CONSTANTS
pub const TILE_SIZE: f32 = 16.0;
pub const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_HEIGHT: f32 = 36.0;
pub const GRAVITY: f32 = -1422.0;
const JUMP_BUFFER_TIME: f64 = 0.1; // seconds before touching ground that jump will be valid

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_event::<events::JumpEvent>()
            .add_event::<events::LandingEvent>()
            .add_event::<events::CeilHitEvent>()
            .add_event::<events::DirectionChangeEvent>()
            .init_resource::<Animations>()
            .insert_resource(PlayerPositionsRes { value: vec![] })
            .insert_resource(StatsRes {
                value: Stats::from_config(
                    SkinColor::Light,
                    MentalHealth::Healthy,
                    true,
                    Intelligence::Smart,
                    true,
                    Strength::Weak,
                    Wealth::Rich,
                ),
            })
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(ObstaclesRes {
                map: HashMap::new(),
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(2)
            .register_ldtk_int_cell::<OneWayPlatformBundle>(3)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup::setup_game.label("setup_game"))
                    .with_system(setup::show_character_menu.after("setup_game")),
            )
            .add_system_set(
                // Runs while showing character menu (extra bg setup)
                SystemSet::on_inactive_update(GameState::Game)
                    .with_system(setup::setup_obstacles)
                    .with_system(setup::setup_entities.label("setup_entities")),
            )
            .add_system_set(SystemSet::on_resume(GameState::Game).with_system(goo::setup_goo))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(player::player_color.label("player_color"))
                    .with_system(depression::show_depressed_text.after("player_color"))
                    .with_system(input::handle_input.label("input").after("player_color"))
                    .with_system(player::player_movement.label("movement").after("input"))
                    .with_system(player::player_animation.after("movement"))
                    .with_system(vfx::add_jump_dust.after("movement"))
                    .with_system(sfx::play_jump_sound.after("movement"))
                    .with_system(vfx::add_landing_dust.after("movement"))
                    .with_system(vfx::add_direction_change_dust.after("movement"))
                    .with_system(sfx::play_landing_sound.after("movement"))
                    .with_system(vfx::add_ceil_hit_sprite.after("movement"))
                    .with_system(sfx::play_ceil_hit_sound.after("movement"))
                    .with_system(vfx::remove_vfx)
                    .with_system(camera::camera_movement.after("movement"))
                    .with_system(goo::goo_movement.label("goo_movement"))
                    .with_system(goo::goo_collision.after("goo_movement").after("movement"))
                    .with_system(goal::goal_collision.after("movement"))
                    .with_system(depression::trigger_depression)
                    .with_system(vfx::blink_player)
                    .with_system(bouncer::bounce_player),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(clean_state::<GameStateEntity>),
            );
    }
}

// RESOURCES
#[derive(Default)]
pub struct Animations {
    idle: Handle<SpriteSheetAnimation>,
    run: Handle<SpriteSheetAnimation>,
    jump: Vec<Handle<SpriteSheetAnimation>>,
    jump_dust: Handle<SpriteSheetAnimation>,
    landing_dust: Handle<SpriteSheetAnimation>,
    dust_atlas: Handle<TextureAtlas>,
}

pub struct ObstaclesRes {
    map: HashMap<Point<i32>, Obstacle>,
}

#[derive(Debug)]
pub struct PlayerPositionsRes {
    pub value: Vec<Transform>,
}

// COMPONENTS
#[derive(Component)]
struct GameStateEntity;

#[derive(Debug, PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    direction: PlayerDirection,
    depressed_until: f64,
    blink_until: f64,
    lifes: i32,
    bounce_force: Option<f32>,
    last_ground_time: Option<f64>,
    buffer_jump_time: Option<f64>,
}

impl Player {
    fn blink(&mut self, seconds_since_startup: f64, blink_duration_secs: Option<f64>) {
        let blink_duration_secs = blink_duration_secs.unwrap_or(1.5);
        self.blink_until = seconds_since_startup + blink_duration_secs;
    }

    fn is_buffered_jump_valid(&self, seconds_since_startup: f64) -> bool {
        if let Some(buffer_jump_time) = self.buffer_jump_time {
            (seconds_since_startup - buffer_jump_time) < JUMP_BUFFER_TIME
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct Position {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct OneWayPlatform;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct OneWayPlatformBundle {
    one_way_platform: OneWayPlatform,
}

#[derive(Component)]
pub struct LifesText;

#[derive(Component)]
pub struct DepressedText;
