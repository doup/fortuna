use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkWorldBundle, LevelEvent};
use std::time::Duration;

use crate::{
    loading::{GameAssets, UIAssets},
    stats::{StatsRes, Wealth},
    GameState,
};

use super::{
    bouncer, camera, goal,
    obstacles::{Obstacle, Point},
    Animations, DepressedText, GameStateEntity, LifesText, ObstaclesRes, OneWayPlatform, Player,
    PlayerDirection, PlayerPositionsRes, Position, Velocity, Wall,
};

pub fn show_character_menu(mut app_state: ResMut<State<GameState>>) {
    app_state.push(GameState::CharacterMenu).unwrap();
}

pub fn setup_game(
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
                        value: stats.value.lifes.to_string(),
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

pub fn setup_obstacles(
    mut obstacles: ResMut<ObstaclesRes>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    one_way_platforms: Query<&GridCoords, With<OneWayPlatform>>,
) {
    for event in level_events.iter() {
        if let LevelEvent::Transformed(_) = event {
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
    }
}

pub fn setup_animations(
    game_assets: Res<GameAssets>,
    mut animations: ResMut<Animations>,
    mut animation_sheets: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let frame_duration = Duration::from_millis(30);

    animations.idle = animation_sheets.add(SpriteSheetAnimation::from_range(0..=0, frame_duration));

    animations.run = animation_sheets.add(SpriteSheetAnimation::from_range(1..=24, frame_duration));

    animations.vfx_atlas = game_assets.vfx_atlas.clone();

    animations.vfx_debug = animation_sheets
        .add(SpriteSheetAnimation::from_range(0..=0, Duration::from_millis(1000)).once());

    animations.vfx_run_jump_dust =
        animation_sheets.add(SpriteSheetAnimation::from_range(1..=11, frame_duration).once());

    animations.vfx_ceil_hit =
        animation_sheets.add(SpriteSheetAnimation::from_range(12..=18, frame_duration).once());

    animations.vfx_landing_dust =
        animation_sheets.add(SpriteSheetAnimation::from_range(19..=36, frame_duration).once());

    animations.jump = vec![
        animation_sheets.add(SpriteSheetAnimation::from_range(25..=25, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(26..=26, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(27..=27, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(28..=28, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(29..=29, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(30..=30, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(31..=31, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(32..=32, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(33..=33, frame_duration)),
        animation_sheets.add(SpriteSheetAnimation::from_range(34..=34, frame_duration)),
    ];
}

fn get_entities<'a>(
    entities: &'a Query<(&Transform, &EntityInstance), Added<EntityInstance>>,
    identifier: &str,
) -> Vec<(&'a Transform, &'a EntityInstance)> {
    entities
        .iter()
        .filter(|(_, instance)| instance.identifier == identifier)
        .collect::<Vec<_>>()
}

pub fn setup_entities(
    stats: Res<StatsRes>,
    game_assets: Res<GameAssets>,
    animations: Res<Animations>,
    mut player_positions: ResMut<PlayerPositionsRes>,
    mut commands: Commands,
    entities: Query<(&Transform, &EntityInstance), Added<EntityInstance>>,
) {
    let player_entities = get_entities(&entities, "Player");

    if !player_entities.is_empty() {
        // Prepare Player Positions Resource
        player_positions.value = player_entities
            .iter()
            .map(|(transform, _)| *(*transform))
            .collect::<Vec<_>>();

        // Sort DESC by translation.y
        player_positions.value.sort_by(|a_transform, b_transform| {
            b_transform
                .translation
                .y
                .partial_cmp(&a_transform.translation.y)
                .unwrap()
        });

        // Create player and move to position
        let pos = match stats.value.wealth {
            Wealth::Rich => 0,
            Wealth::MiddleClass => 1,
            Wealth::Poor => 2,
        };

        let &transform = player_positions.value.get(pos).unwrap();

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_assets.player_atlas.clone(),
                ..Default::default()
            })
            .insert(animations.idle.clone())
            .insert(Play)
            .insert(Player {
                direction: PlayerDirection::Right,
                depressed_until: 0.0,
                blink_until: 0.0,
                last_ground_time: None,
                lifes: stats.value.lifes,
                bounce_force: None,
                buffer_jump_time: None,
            })
            .insert(Position {
                value: transform.translation.truncate(),
            })
            .insert(Velocity { x: 0.0, y: 0.0 })
            .insert(GameStateEntity);
    }

    let goal_entities = get_entities(&entities, "Goal");

    if !goal_entities.is_empty() {
        let (goal_transform, goal_entity) = goal_entities[0];

        commands
            .spawn_bundle(SpriteBundle {
                visibility: Visibility { is_visible: false },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        goal_entity.width as f32,
                        goal_entity.height as f32,
                    )),
                    ..Default::default()
                },
                transform: Transform {
                    translation: goal_transform.translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(goal::Goal)
            .insert(GameStateEntity);
    }

    let bouncer_entities = get_entities(&entities, "Bouncer");

    for bouncer_entity in bouncer_entities {
        let (bouncer_transform, bouncer_entity) = bouncer_entity;

        commands
            .spawn_bundle(SpriteBundle {
                visibility: Visibility { is_visible: false },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        bouncer_entity.width as f32,
                        bouncer_entity.height as f32,
                    )),
                    ..Default::default()
                },
                transform: Transform {
                    translation: bouncer_transform.translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(bouncer::get_bouncer_from_entity_instance(bouncer_entity))
            .insert(GameStateEntity);
    }
}
