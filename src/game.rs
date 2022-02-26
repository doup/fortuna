use crate::{loading::GameAssets, GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(setup_entities) // TODO: For some reason I'm unable to run this just after `setup_game` on `on_enter` and query for `Added<EntityInstance>`
                    .with_system(handle_input)
                    .with_system(player_movement)
                    .with_system(camera_movement),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_game));
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

// SYSTEMS
fn setup_game(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameStateEntity);

    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: game_assets.map.clone(),
            ..Default::default()
        })
        .insert(GameStateEntity);
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
        let (&transform, entity_instance) = player_entities
            .get(rand::thread_rng().gen_range(0..player_entities.len()))
            .unwrap();

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(16.0, 32.0)),
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
    mut query: Query<(&mut Acceleration, &mut Velocity, &mut Transform), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut acceleration, mut velocity, mut transform) = query.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        let is_grounded = acceleration.y.abs() == 0.0 && velocity.y.abs() == 0.0;

        if is_grounded {
            acceleration.y = 0.0;
            velocity.y = 35.0;
        }
    }

    if keys.just_pressed(KeyCode::R) {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        velocity.x = 0.0;
        velocity.y = 0.0;
        acceleration.x = 0.0;
        acceleration.y = 0.0;
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x = -8.0;
    } else if keys.pressed(KeyCode::Right) {
        velocity.x = 8.0;
    } else {
        velocity.x = 0.0;
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Acceleration, &mut Velocity, &mut Transform), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut acceleration, mut velocity, mut transform) = query.single_mut();

    acceleration.y -= time.delta_seconds() * 20.0;
    velocity.x += acceleration.x;
    velocity.y += acceleration.y;
    transform.translation.x += velocity.x;
    transform.translation.y += velocity.y;

    if transform.translation.y < 32.0 {
        transform.translation.y = 32.0;
    }
}

fn camera_movement(
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    if !players.is_empty() && !cameras.is_empty() {
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
