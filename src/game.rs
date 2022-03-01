use std::collections::HashMap;

use crate::{loading::GameAssets, GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(ObstaclesRes {
                map: HashMap::new(),
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(2)
            .register_ldtk_int_cell::<OneWayPlatformBundle>(4)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(setup_obstacles)
                    .with_system(setup_entities.label("setup_entities")) // TODO: For some reason I'm unable to run this just after `setup_game` on `on_enter` and query for `Added<EntityInstance>`
                    .with_system(handle_input.label("input").after("setup_entities"))
                    .with_system(player_movement.label("movement").after("input"))
                    .with_system(camera_movement.after("movement")),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_game));
    }
}

// RESOURCES
const SKIN_WIDTH: f32 = 2.0;
const TILE_SIZE: f32 = 16.0;
const PLAYER_WIDTH: f32 = 16.0;
const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
const PLAYER_HEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;

struct ObstaclesRes {
    map: HashMap<Point<i32>, Obstacle>,
}

#[derive(Debug)]
struct Obstacle {
    pos: Point<i32>,
    is_one_way: bool,
}

#[derive(Debug, PartialEq, Hash, Eq)]
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
struct Player;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Acceleration {
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

// SYSTEMS
fn setup_game(mut commands: Commands, game_assets: Res<GameAssets>) {
    let camera = OrthographicCameraBundle::new_2d();

    commands
        .spawn_bundle(OrthographicCameraBundle {
            orthographic_projection: OrthographicProjection {
                scale: 0.5,
                ..camera.orthographic_projection
            },
            ..camera
        })
        .insert(GameStateEntity);

    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: game_assets.map.clone(),
            ..Default::default()
        })
        .insert(GameStateEntity);
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
    mut commands: Commands,
    entities: Query<(&Transform, &EntityInstance), Added<EntityInstance>>,
) {
    let player_entities = entities
        .iter()
        .filter(|(_, instance)| instance.identifier == "Player")
        .collect::<Vec<_>>();

    if player_entities.len() > 0 {
        let (&transform, _) = player_entities
            .get(rand::thread_rng().gen_range(0..player_entities.len()))
            .unwrap();

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..Default::default()
                },
                transform: transform.clone(),
                ..Default::default()
            })
            .insert(Player)
            .insert(Velocity { x: 0.0, y: 0.0 })
            .insert(Acceleration { x: 0.0, y: 0.0 })
            .insert(GameStateEntity);
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut query: Query<(&mut Acceleration, &mut Velocity), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut acceleration, mut velocity) = query.single_mut();
    let is_grounded = acceleration.y.abs() == 0.0 && velocity.y.abs() == 0.0;

    if keys.just_pressed(KeyCode::Space) {
        if is_grounded {
            acceleration.y = 0.0;
            velocity.y = 10.0;
        }
    }

    if keys.just_pressed(KeyCode::R) {
        app_state.set(GameState::MainMenu).unwrap();
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x = if is_grounded { -6.0 } else { -4.0 };
    } else if keys.pressed(KeyCode::Right) {
        velocity.x = if is_grounded { 6.0 } else { 4.0 };
    } else {
        velocity.x = 0.0;
    }
}

fn player_movement(
    time: Res<Time>,
    obstacles: Res<ObstaclesRes>,
    mut player_query: Query<(&mut Acceleration, &mut Velocity, &mut Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut acceleration, mut velocity, mut transform) = player_query.single_mut();

    let gravity = 20.0;
    acceleration.y -= time.delta_seconds() * gravity;

    let velocity_x = velocity.x + acceleration.x;
    let velocity_y = velocity.y + acceleration.y;
    let is_moving_right = velocity_x > 0.0;
    let is_moving_up = velocity_y > 0.0;

    if velocity_x != 0.0 {
        let pos_x = transform.translation.x + velocity_x;
        let bottom = transform.translation.y - PLAYER_HEIGHT_HALF + SKIN_WIDTH;
        let top = transform.translation.y + PLAYER_HEIGHT_HALF - SKIN_WIDTH;

        let horizontal_bbox = if is_moving_right {
            let left = transform.translation.x + PLAYER_WIDTH_HALF;
            let right = left + velocity_x.abs();
            BBox::new((left, bottom), (right, top))
        } else {
            let right = transform.translation.x - PLAYER_WIDTH_HALF;
            let left = right - velocity_x.abs();
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

    if velocity_y != 0.0 {
        let pos_y = transform.translation.y + velocity_y;
        let left = transform.translation.x - PLAYER_WIDTH_HALF + SKIN_WIDTH;
        let right = transform.translation.x + PLAYER_WIDTH_HALF - SKIN_WIDTH;

        let vertical_bbox = if is_moving_up {
            let bottom = transform.translation.y + PLAYER_HEIGHT_HALF;
            let top = bottom + velocity_y.abs();
            BBox::new((left, bottom), (right, top))
        } else {
            let top = transform.translation.y - PLAYER_HEIGHT_HALF;
            let bottom = top - velocity_y.abs();
            BBox::new((left, bottom), (right, top))
        };

        let vertical_obstacles = get_obstacle_list(
            get_tile_list(get_tile_space_bbox(&vertical_bbox)),
            &obstacles.map,
            is_moving_up,
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
                acceleration.y = 0.0;
                velocity.y = 0.0;
            } else {
                transform.translation.y = pos_y;
            };
        } else {
            transform.translation.y = pos_y;
        }
    }
}

fn camera_movement(
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    if !players.is_empty() {
        let player_transform = players.single();
        let mut camera_transform = cameras.single_mut();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
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
