mod bouncer;
mod camera;
mod goal;
mod goo;

use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;
use std::{collections::HashMap, time::Duration};

use crate::{
    loading::{GameAssets, UIAssets},
    stats::{
        SkinColor, Stats, StatsRes, Wealth, JUMP_HEIGHT_DEPRESSED_PX, MAX_DEPRE_DURATION,
        MAX_DEPRE_DURATION, MIN_DEPRE_DURATION, MIN_TIME_BETWEEN_DEPRE,
    },
    GameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .init_resource::<Animations>()
            .insert_resource(PlayerPositionsRes(vec![]))
            .insert_resource(StatsRes(Stats::new()))
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(ObstaclesRes {
                map: HashMap::new(),
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(2)
            .register_ldtk_int_cell::<OneWayPlatformBundle>(3)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_game.label("setup_game"))
                    .with_system(show_character_menu.after("setup_game")),
            )
            .add_system_set(
                // Runs while showing character menu (extra bg setup)
                SystemSet::on_inactive_update(GameState::Game)
                    .with_system(setup_obstacles)
                    .with_system(setup_entities.label("setup_entities")),
            )
            .add_system_set(SystemSet::on_resume(GameState::Game).with_system(setup_goo))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(player_color.label("player_color"))
                    .with_system(show_depressed_text.after("player_color"))
                    .with_system(handle_input.label("input").after("player_color"))
                    .with_system(player_movement.label("movement").after("input"))
                    .with_system(player_animation.after("movement"))
                    .with_system(camera::camera_movement.after("movement"))
                    .with_system(goo::goo_movement.label("goo_movement"))
                    .with_system(goo::goo_collision.after("goo_movement").after("movement"))
                    .with_system(goal::goal_collision.after("movement"))
                    .with_system(trigger_depression)
                    .with_system(blink_player)
                    .with_system(bouncer::bounce_player),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_game));
    }
}

// PLAYER CONSTANTS
const TILE_SIZE: f32 = 16.0;
const SKIN_SIZE: f32 = 2.0;
const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
const PLAYER_HEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;
const PLAYER_BLINK_DURATION: f64 = 1.5;

// JUMP
pub const GRAVITY: f32 = -1422.0;
const COYOTE_TIME: f64 = 0.125; // seconds after falling from a platform that still can jump
const JUMP_BUFFER_TIME: f64 = 0.1; // seconds before touching ground that jump will be valid

// RESOURCES
#[derive(Default)]
struct Animations {
    idle: Handle<SpriteSheetAnimation>,
    run: Handle<SpriteSheetAnimation>,
}

pub struct ObstaclesRes {
    map: HashMap<Point<i32>, Obstacle>,
}

#[derive(Debug)]
pub struct PlayerPositionsRes(pub Vec<Transform>);

#[derive(Debug)]
struct Obstacle {
    pos: Point<i32>,
    is_one_way: bool,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Point<T: Copy>(T, T);

#[derive(Debug, PartialEq)]
struct BBox<T: Copy> {
    min: Point<T>,
    max: Point<T>,
}

impl<T: Copy> BBox<T> {
    fn new(min: (T, T), max: (T, T)) -> BBox<T> {
        BBox {
            min: Point(min.0, min.1),
            max: Point(max.0, max.1),
        }
    }

    fn left(&self) -> T {
        self.min.0
    }

    fn right(&self) -> T {
        self.max.0
    }

    fn top(&self) -> T {
        self.max.1
    }

    fn bottom(&self) -> T {
        self.min.1
    }
}

// COMPONENTS
#[derive(Component)]
struct GameStateEntity;

#[derive(PartialEq)]
enum PlayerDirection {
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

#[derive(Component)]
pub struct Position {
    pub value: Vec3,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
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

// SYSTEMS
fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    ui_assets: Res<UIAssets>,
    stats: Res<StatsRes>,
) {
    let camera = OrthographicCameraBundle::new_2d();

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(GameStateEntity);

    commands
        .spawn_bundle(OrthographicCameraBundle {
            orthographic_projection: OrthographicProjection {
                scale: 0.5,
                ..camera.orthographic_projection
            },
            ..camera
        })
        .insert(camera::GameCamera)
        .insert(GameStateEntity);

    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: game_assets.map.clone(),
            ..Default::default()
        })
        .insert(GameStateEntity);

    // Add depressed message
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "I don't want to keep jumping...",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 10.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(GameStateEntity)
        .insert(DepressedText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Lifes: ".to_string(),
                        style: TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: stats.0.lifes.to_string(),
                        style: TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::TOMATO,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameStateEntity)
        .insert(LifesText);
}

fn setup_goo(mut commands: Commands, time: Res<Time>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(goo::Goo::new(time.seconds_since_startup()))
        .insert(GameStateEntity);
}

fn show_character_menu(mut app_state: ResMut<State<GameState>>) {
    app_state.push(GameState::CharacterMenu).unwrap();
}

fn setup_obstacles(
    mut obstacles: ResMut<ObstaclesRes>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    one_way_platforms: Query<&GridCoords, With<OneWayPlatform>>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::Transformed(_) => {
                obstacles.map.clear();

                walls.for_each(|grid_coords| {
                    obstacles.map.insert(
                        Point(grid_coords.x, grid_coords.y),
                        Obstacle {
                            pos: Point(grid_coords.x, grid_coords.y),
                            is_one_way: false,
                        },
                    );
                });

                one_way_platforms.for_each(|grid_coords| {
                    obstacles.map.insert(
                        Point(grid_coords.x, grid_coords.y),
                        Obstacle {
                            pos: Point(grid_coords.x, grid_coords.y),
                            is_one_way: true,
                        },
                    );
                });
            }
            _ => (),
        }
    }
}

fn setup_entities(
    stats: Res<StatsRes>,
    game_assets: Res<GameAssets>,
    mut animations: ResMut<Animations>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animation_sheets: ResMut<Assets<SpriteSheetAnimation>>,
    mut player_positions: ResMut<PlayerPositionsRes>,
    mut commands: Commands,
    entities: Query<(&Transform, &EntityInstance), Added<EntityInstance>>,
) {
    let player_entities = entities
        .iter()
        .filter(|(_, instance)| instance.identifier == "Player")
        .collect::<Vec<_>>();

    if player_entities.len() > 0 {
        // Prepare Player Positions Resource
        player_positions.0 = player_entities
            .iter()
            .map(|(transform, _)| *transform.clone())
            .collect::<Vec<_>>();

        // Sort DESC by translation.y
        player_positions.0.sort_by(|a_transform, b_transform| {
            b_transform
                .translation
                .y
                .partial_cmp(&a_transform.translation.y)
                .unwrap()
        });

        // Create player and move to position
        let pos = match stats.0.wealth {
            Wealth::Rich => 0,
            Wealth::MiddleClass => 1,
            Wealth::Poor => 2,
        };

        let &transform = player_positions.0.get(pos).unwrap();

        // Animation
        animations.idle = animation_sheets.add(SpriteSheetAnimation::from_range(
            0..=0,
            Duration::from_millis(30),
        ));

        animations.run = animation_sheets.add(SpriteSheetAnimation::from_range(
            1..=24,
            Duration::from_millis(30),
        ));

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.add(TextureAtlas::from_grid(
                    game_assets.player_anim.clone(),
                    Vec2::new(48.0, 48.0),
                    25,
                    1,
                )),
                // sprite: Sprite {
                //     color: Color::BLACK,
                //     custom_size: Some(Vec2::new(48.0, 48.0)),
                //     ..Default::default()
                // },
                ..Default::default()
            })
            .insert(animations.idle.clone())
            .insert(Play)
            .insert(Player {
                direction: PlayerDirection::Right,
                depressed_until: 0.0,
                blink_until: 0.0,
                last_ground_time: None,
                lifes: stats.0.lifes,
                bounce_force: None,
                buffer_jump_time: None,
            })
            .insert(Position {
                value: transform.translation.clone(),
            })
            .insert(Velocity { x: 0.0, y: 0.0 })
            .insert(GameStateEntity);
    }

    let goal_entities = entities
        .iter()
        .filter(|(_, instance)| instance.identifier == "Goal")
        .collect::<Vec<_>>();

    if goal_entities.len() > 0 {
        let (goal_transform, goal_entity) = goal_entities[0];

        commands
            .spawn_bundle(SpriteBundle {
                visibility: Visibility { is_visible: false },
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(
                        goal_entity.width as f32,
                        goal_entity.height as f32,
                    )),
                    ..Default::default()
                },
                transform: Transform {
                    translation: goal_transform.translation.clone(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(goal::Goal)
            .insert(GameStateEntity);
    }

    let bouncer_entities = entities
        .iter()
        .filter(|(_, instance)| instance.identifier == "Bouncer")
        .collect::<Vec<_>>();

    for bouncer_entity in bouncer_entities {
        let (bouncer_transform, bouncer_entity) = bouncer_entity;

        commands
            .spawn_bundle(SpriteBundle {
                visibility: Visibility { is_visible: false },
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(
                        bouncer_entity.width as f32,
                        bouncer_entity.height as f32,
                    )),
                    ..Default::default()
                },
                transform: Transform {
                    translation: bouncer_transform.translation.clone(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(bouncer::get_bouncer_from_entity_instance(&bouncer_entity))
            .insert(GameStateEntity);
    }
}

fn player_color(
    stats: Res<StatsRes>,
    mut player_query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    // let mut sprite = player_query.single_mut();

    // sprite.color = match stats.0.color {
    //     SkinColor::Light => Color::hex("b8ddf5").unwrap(),
    //     SkinColor::Medium => Color::hex("3f789d").unwrap(),
    //     SkinColor::Dark => Color::hex("103954").unwrap(),
    // };
}

fn show_depressed_text(
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

fn handle_input(
    stats: Res<StatsRes>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut player_query: Query<(&mut Velocity, &mut Player), With<Player>>,
) {
    let (mut velocity, mut player) = player_query.single_mut();
    let time_delta = time.delta_seconds();
    let top_speed;
    let top_speed_rate;
    let stop_rate;
    let jump_force;

    if player.depressed_until > time.seconds_since_startup() {
        top_speed = stats.0.top_speed_depressed;
        top_speed_rate = stats.0.top_speed_rate_depressed;
        stop_rate = stats.0.stop_rate_depressed;
        jump_force = stats.0.jump_force_depressed;
    } else {
        top_speed = stats.0.top_speed;
        top_speed_rate = stats.0.top_speed_rate;
        stop_rate = stats.0.stop_rate;
        jump_force = stats.0.jump_force;
    }

    if let Some(last_ground_time) = player.last_ground_time {
        // Player is in the ground
        let jump_force = jump_force - GRAVITY * time_delta; // Correct by gravity… why?? I don't remember :facepalm:
        let is_in_jump_window = time.seconds_since_startup() < last_ground_time + COYOTE_TIME;
        let is_buffered_jump_valid = if let Some(buffer_jump_time) = player.buffer_jump_time {
            (time.seconds_since_startup() - buffer_jump_time) < JUMP_BUFFER_TIME
        } else {
            false
        };
        let can_jump =
            keys.just_pressed(KeyCode::Space) && is_in_jump_window || is_buffered_jump_valid;

        if can_jump {
            velocity.y = jump_force;
            player.last_ground_time = None;
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
        player.direction = PlayerDirection::Left;
        velocity.x = (velocity.x - top_speed_rate * time_delta).max(-top_speed);
    } else if keys.pressed(KeyCode::Right) {
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

fn player_movement(
    stats: Res<StatsRes>,
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
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
            BBox::new((left, bottom), (right, top))
        } else {
            let right = position.value.x - PLAYER_WIDTH_HALF;
            let left = right - velocity.x.abs() * time_delta;
            BBox::new((left, bottom), (right, top))
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
            BBox::new((left, bottom), (right, top))
        } else {
            let top = position.value.y - PLAYER_HEIGHT_HALF;
            let bottom = top - velocity.y.abs() * time_delta;
            BBox::new((left, bottom), (right, top))
        };

        let vertical_obstacles = get_obstacle_list(
            get_tile_list(get_tile_space_bbox(&vertical_bbox)),
            &obstacles.map,
            is_moving_up && stats.0.can_skip_one_way_platforms,
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
                    velocity.y = 0.0;
                    player.last_ground_time = Some(time.seconds_since_startup());
                } else {
                    velocity.y = -velocity.y * 0.1;
                }
            } else {
                position.value.y = pos_y;
            };
        } else {
            position.value.y = pos_y;
        }
    }
}

fn player_animation(
    animations: Res<Animations>,
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
    let is_moving = velocity.x != 0.0;

    sprite_transform.translation.x = position.value.x;
    sprite_transform.translation.y = position.value.y + 8.0;
    sprite_transform.translation.z = 10.0;

    sprite.flip_x = player.direction == PlayerDirection::Left;

    if is_grounded && is_moving {
        *animation = animations.run.clone();
    } else {
        *animation = animations.idle.clone();
    }
}

fn trigger_depression(stats: Res<StatsRes>, time: Res<Time>, mut players: Query<&mut Player>) {
    let mut player = players.single_mut();

    let can_get_depressed = stats.0.is_depressive
        && (player.depressed_until + MIN_TIME_BETWEEN_DEPRE) < time.seconds_since_startup();

    if can_get_depressed {
        if rand::thread_rng().gen_range(0.0..1.0) < stats.0.depre_chance {
            player.depressed_until = time.seconds_since_startup()
                + rand::thread_rng().gen_range(MIN_DEPRE_DURATION..MAX_DEPRE_DURATION);
        }
    }
}

fn blink_player(time: Res<Time>, mut player_query: Query<(&Player, &mut Visibility)>) {
    let (player, mut visibility) = player_query.single_mut();
    let time_seconds = time.seconds_since_startup();

    if player.blink_until > time_seconds {
        visibility.is_visible = (time_seconds * 10.0) as i32 % 2 == 0;
    } else {
        visibility.is_visible = true;
    }
}

fn clean_game(mut commands: Commands, entities: Query<Entity, With<GameStateEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

/// Map screen-space bbox to tile-space bbox
fn get_tile_space_bbox(bbox: &BBox<f32>) -> BBox<i32> {
    BBox::new(
        (
            (bbox.min.0 / TILE_SIZE).floor() as i32,
            (bbox.min.1 / TILE_SIZE).floor() as i32,
        ),
        (
            (bbox.max.0 / TILE_SIZE).floor() as i32,
            (bbox.max.1 / TILE_SIZE).floor() as i32,
        ),
    )
}

/// Map tile-space bbox to list of tile coords
fn get_tile_list(bbox: BBox<i32>) -> Vec<Point<i32>> {
    let mut tiles = Vec::new();

    for y in bbox.bottom()..(bbox.top() + 1) {
        for x in bbox.left()..(bbox.right() + 1) {
            tiles.push(Point(x, y));
        }
    }

    tiles
}

/// Convert screen-space Point to tile-space
fn to_tile_space(pos: Point<f32>) -> Point<i32> {
    Point(
        (pos.0 / TILE_SIZE).floor() as i32,
        (pos.1 / TILE_SIZE).floor() as i32,
    )
}

fn get_first_obstacle_pos_downward(
    obstacles: &HashMap<Point<i32>, Obstacle>,
    pos: Point<i32>,
) -> Option<Point<i32>> {
    for y in (0..pos.1).rev() {
        let obs = obstacles.get(&Point(pos.0, y));

        if obs.is_some() {
            return Some(obs.unwrap().pos.clone());
        }
    }

    None
}

/// Given a tile list, get the ones with obstacles
fn get_obstacle_list(
    tiles: Vec<Point<i32>>,
    obstacles: &HashMap<Point<i32>, Obstacle>,
    ignore_one_way: bool,
) -> Vec<&Obstacle> {
    tiles
        .iter()
        .filter_map(|p| obstacles.get(p))
        .filter(|o| {
            if ignore_one_way {
                return !o.is_one_way;
            }

            true
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_tile_space_bbox() {
        assert_eq!(
            get_tile_space_bbox(&BBox::new((1.0, 1.0), (24.0, 38.0))),
            BBox::new((0, 0), (1, 2))
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new((0.0, 0.0), (16.0, 16.0))),
            BBox::new((0, 0), (1, 1))
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new((0.0, 0.0), (15.0, 15.0))),
            BBox::new((0, 0), (0, 0))
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new((0.0, 0.0), (70.0, 8.0))),
            BBox::new((0, 0), (4, 0))
        );
    }

    #[test]
    fn test_get_tile_list() {
        assert_eq!(
            get_tile_list(BBox::new((0, 0), (1, 2))),
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(0, 1),
                Point(1, 1),
                Point(0, 2),
                Point(1, 2)
            ]
        );

        assert_eq!(get_tile_list(BBox::new((0, 0), (0, 0))), vec![Point(0, 0)]);
    }

    #[test]
    fn test_get_obstacle_list() {
        let mut obstacles = HashMap::new();

        obstacles.insert(
            Point(0, 1),
            Obstacle {
                pos: Point(0, 1),
                is_one_way: false,
            },
        );

        obstacles.insert(
            Point(1, 1),
            Obstacle {
                pos: Point(1, 1),
                is_one_way: true,
            },
        );

        let list = get_obstacle_list(get_tile_list(BBox::new((0, 0), (1, 2))), &obstacles, false);

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].pos, Point(0, 1));
        assert_eq!(list[1].pos, Point(1, 1));

        let list = get_obstacle_list(get_tile_list(BBox::new((0, 0), (1, 2))), &obstacles, true);

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].pos, Point(0, 1));
    }
}
