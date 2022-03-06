use std::collections::HashMap;

use crate::{
    loading::{GameAssets, UIAssets},
    stats::{
        SkinColor, Stats, StatsRes, Wealth, JUMP_HEIGHT_DEPRESSED_PX, MAX_DEPRE_DURATION,
        MIN_DEPRE_DURATION, MIN_TIME_BETWEEN_DEPRE, RUN_STOP_RATE_DEPRESSED,
        RUN_TOP_SPEED_DEPRESSED, RUN_TOP_SPEED_RATE_DEPRESSED,
    },
    GameState,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
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
                    .with_system(camera_movement.after("movement"))
                    .with_system(goo_movement.label("goo_movement"))
                    .with_system(goo_collision.after("goo_movement").after("movement"))
                    .with_system(goal_collision.after("movement"))
                    .with_system(trigger_depression)
                    .with_system(blink_player)
                    .with_system(bounce_player),
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
const GRAVITY: f32 = -1422.0;
// const COYOTE_TIME: f32 = 80.0; // ms after falling a platform that still can jump
// const JUMP_BUFFER_TIME: f32 = 80.0; // ms before touching ground that can be jumped

// GOO
const GOO_INITIAL_POS: f32 = -30.0;
const GOO_SPEED: f32 = 40.0;
const GOO_SIN_AMPLITUDE: f32 = 6.0;
const GOO_HIT_REGRESS: f32 = 50.0;

// BOUNCER
const BOUNCER_FORCE: f32 = 2500.0;
const BOUNCER_DURATION: f32 = 0.5;

// RESOURCES
struct ObstaclesRes {
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

#[derive(Component)]
pub struct Player {
    depressed_until: f64,
    blink_until: f64,
    lifes: i32,
    bounce_force: Option<f32>,
}

#[derive(Component)]
struct Goo {
    y: f32,
    start_time: f32,
    regress: f32,
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

#[derive(Debug)]
enum BouncerType {
    WealthRich,
    SkinColorLight,
}

#[derive(Debug, Component)]
struct Bouncer {
    allow: BouncerType,
    direction: f32,
}

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct GameCamera;

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
        .insert(GameCamera)
        .insert(GameStateEntity);

    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: game_assets.map.clone(),
            ..Default::default()
        })
        .insert(GameStateEntity);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                display: Display::None,
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![TextSection {
                    value: "I don't want to keep jumping...".to_string(),
                    style: TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                }],
                ..Default::default()
            },
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
        .insert(Goo {
            y: GOO_INITIAL_POS,
            start_time: time.seconds_since_startup() as f32,
            regress: 0.0,
        })
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

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..Default::default()
                },
                transform: transform.clone(),
                ..Default::default()
            })
            .insert(Player {
                depressed_until: 0.0,
                blink_until: 0.0,
                lifes: stats.0.lifes,
                bounce_force: None,
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
            .insert(Goal)
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
            .insert(get_bouncer_from_entity_instance(&bouncer_entity))
            .insert(GameStateEntity);
    }
}

fn get_bouncer_from_entity_instance(entity: &EntityInstance) -> Bouncer {
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

fn player_color(stats: Res<StatsRes>, mut player_query: Query<&mut Sprite, With<Player>>) {
    let mut sprite = player_query.single_mut();

    sprite.color = match stats.0.color {
        SkinColor::Light => Color::hex("b8ddf5").unwrap(),
        SkinColor::Medium => Color::hex("3f789d").unwrap(),
        SkinColor::Dark => Color::hex("103954").unwrap(),
    };
}

fn show_depressed_text(
    time: Res<Time>,
    mut depressed_text_query: Query<&mut Style, With<DepressedText>>,
    player_query: Query<&Player, With<Player>>,
) {
    let player = player_query.single();
    let mut depressed_text = depressed_text_query.single_mut();

    depressed_text.display = if player.depressed_until > time.seconds_since_startup() {
        Display::Flex
    } else {
        Display::None
    };
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
    let jump_height_px;

    if player.depressed_until > time.seconds_since_startup() {
        top_speed = RUN_TOP_SPEED_DEPRESSED;
        top_speed_rate = RUN_TOP_SPEED_RATE_DEPRESSED;
        stop_rate = RUN_STOP_RATE_DEPRESSED;
        jump_height_px = JUMP_HEIGHT_DEPRESSED_PX;
    } else {
        top_speed = stats.0.top_speed;
        top_speed_rate = stats.0.top_speed_rate;
        stop_rate = stats.0.stop_rate;
        jump_height_px = stats.0.jump_height_px;
    }

    let jump_force = (-2.0 * GRAVITY * jump_height_px).sqrt() - GRAVITY * time_delta;
    let is_grounded = velocity.y == 0.0;
    let is_coyote_time = velocity.y < 0.0 && velocity.y > -150.0; // Naive implementation

    if keys.just_pressed(KeyCode::Space) {
        if is_grounded || is_coyote_time {
            velocity.y = jump_force;
        }
    }

    if keys.just_pressed(KeyCode::R) {
        app_state.set(GameState::MainMenu).unwrap();
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x = (velocity.x - top_speed_rate * time_delta).max(-top_speed);
    } else if keys.pressed(KeyCode::Right) {
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
            (bounce_force - (BOUNCER_FORCE / BOUNCER_DURATION) * time_delta).max(0.0)
        } else if bounce_force < 0.0 {
            (bounce_force + (BOUNCER_FORCE / BOUNCER_DURATION) * time_delta).min(0.0)
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
    mut player_query: Query<(&mut Velocity, &mut Transform, &Player), With<Player>>,
) {
    let (mut velocity, mut transform, player) = player_query.single_mut();
    let time_delta = time.delta_seconds();

    velocity.y += GRAVITY * time_delta;

    let is_moving_right = velocity.x > 0.0;
    let is_moving_up = velocity.y > 0.0;

    if velocity.x != 0.0 {
        let pos_x = transform.translation.x + velocity.x * time_delta;
        let bottom = transform.translation.y - PLAYER_HEIGHT_HALF + SKIN_SIZE;
        let top = transform.translation.y + PLAYER_HEIGHT_HALF - SKIN_SIZE;

        let horizontal_bbox = if is_moving_right {
            let left = transform.translation.x + PLAYER_WIDTH_HALF;
            let right = left + velocity.x.abs() * time_delta;
            BBox::new((left, bottom), (right, top))
        } else {
            let right = transform.translation.x - PLAYER_WIDTH_HALF;
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

            transform.translation.x = if is_moving_right {
                pos_x.min(nearest_obstacle_x)
            } else {
                pos_x.max(nearest_obstacle_x)
            };
        } else {
            transform.translation.x = pos_x;
        }
    }

    if velocity.y != 0.0 {
        let pos_y = transform.translation.y + velocity.y * time_delta;
        let left = transform.translation.x - PLAYER_WIDTH_HALF + SKIN_SIZE;
        let right = transform.translation.x + PLAYER_WIDTH_HALF - SKIN_SIZE;

        let vertical_bbox = if is_moving_up {
            let bottom = transform.translation.y + PLAYER_HEIGHT_HALF;
            let top = bottom + velocity.y.abs() * time_delta;
            BBox::new((left, bottom), (right, top))
        } else {
            let top = transform.translation.y - PLAYER_HEIGHT_HALF;
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
                transform.translation.y = nearest_obstacle_y;
                velocity.y = 0.0;
            } else {
                transform.translation.y = pos_y;
            };
        } else {
            transform.translation.y = pos_y;
        }
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

            println!("Depressed… :-(");
            // Add message
        }
    }
}

fn goo_movement(
    time: Res<Time>,
    players: Query<&Transform, (With<Player>, Without<Goo>)>,
    mut goo_query: Query<(&mut Goo, &mut Transform, &Sprite)>,
) {
    let player_transform = players.single();
    let (mut goo, mut transform, sprite) = goo_query.single_mut();

    goo.y = GOO_INITIAL_POS - goo.regress
        + (time.seconds_since_startup() as f32 - goo.start_time) * GOO_SPEED
        + ((time.seconds_since_startup() * 2.0).sin() as f32) * GOO_SIN_AMPLITUDE;

    transform.translation.x = player_transform.translation.x;
    transform.translation.y = goo.y - sprite.custom_size.unwrap().y / 2.0;
    transform.translation.z = 500.0;
}

fn goo_collision(
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
    mut player_query: Query<(&Transform, &mut Player), (With<Player>, Without<Goo>)>,
    mut app_state: ResMut<State<GameState>>,
    mut goo_query: Query<&mut Goo>,
) {
    let mut goo = goo_query.single_mut();
    let (player_transform, mut player) = player_query.single_mut();

    if player_transform.translation.y < goo.y {
        player.lifes -= 1;

        if player.lifes == 0 {
            app_state.set(GameState::LoseMenu).unwrap();
        } else {
            let first_down_obstacle_tile_pos = get_first_obstacle_pos_downward(
                &obstacles.map,
                to_tile_space(Point(
                    player_transform.translation.x,
                    player_transform.translation.y,
                )),
            )
            .unwrap();

            let floor_y = first_down_obstacle_tile_pos.1 as f32 * TILE_SIZE + TILE_SIZE;
            let distance_to_floor = goo.y - floor_y;

            player.blink_until = time.seconds_since_startup() + PLAYER_BLINK_DURATION;
            goo.regress += distance_to_floor + GOO_HIT_REGRESS;
        }
    }
}

fn goal_collision(
    mut app_state: ResMut<State<GameState>>,
    mut player_query: Query<(&Transform, &Sprite), (With<Player>, Without<Goal>)>,
    goal_query: Query<(&Transform, &Sprite), (With<Goal>, Without<Player>)>,
) {
    let (player_transform, player_sprite) = player_query.single_mut();
    let (goal_transform, goal_sprite) = goal_query.single();

    if collide(
        player_transform.translation,
        player_sprite.custom_size.unwrap(),
        goal_transform.translation,
        goal_sprite.custom_size.unwrap(),
    )
    .is_some()
    {
        app_state.set(GameState::WinMenu).unwrap();
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

fn bounce_player(
    stats: Res<StatsRes>,
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &mut Player, &Sprite, &Velocity),
        (With<Player>, Without<Bouncer>),
    >,
    bouncer_query: Query<(&Transform, &Sprite, &Bouncer), (With<Bouncer>, Without<Player>)>,
) {
    let (mut player_transform, mut player, player_sprite, player_velocity) =
        player_query.single_mut();

    for (bouncer_transform, bouncer_sprite, bouncer) in bouncer_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_sprite.custom_size.unwrap(),
            bouncer_transform.translation,
            bouncer_sprite.custom_size.unwrap(),
        );

        let allow = match bouncer.allow {
            BouncerType::SkinColorLight => stats.0.color == SkinColor::Light,
            BouncerType::WealthRich => stats.0.wealth == Wealth::Rich,
        };

        if !allow && collision.is_some() {
            if bouncer.direction == 1.0 {
                player_transform.translation.x += 2.0 * PLAYER_WIDTH;
            } else {
                player_transform.translation.x -= 2.0 * PLAYER_WIDTH;
            }

            if let None = player.bounce_force {
                println!("You're not allowed here: {:?}", bouncer);

                player.bounce_force = Some(BOUNCER_FORCE * bouncer.direction);
                player.blink_until = time.seconds_since_startup() + BOUNCER_DURATION as f64;
            }
        }
    }
}

fn camera_movement(
    players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, With<GameCamera>>,
) {
    let player_transform = players.single();
    let mut camera_transform = cameras.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    // camera_transform.translation.x = 208.0;
    // camera_transform.translation.y = 192.0;
    // println!("{:?}", camera_transform.translation);
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
