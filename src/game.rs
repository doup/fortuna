use crate::GameState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game))
            // .add_system_set(
            //     SystemSet::on_update(GameState::Game)
            //         .with_system(handle_ui_buttons)
            //         .with_system(handle_play_button)
            //         .with_system(handle_reborn_button),
            // )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_game));
    }
}

// COMPONENTS
#[derive(Component)]
struct GameStateEntity;

// SYSTEMS
fn setup_game(mut commands: Commands, mut app_state: ResMut<State<GameState>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameStateEntity);

    println!("Game started… aaaand finished. Moving to Result state.");

    app_state.set(GameState::Score).unwrap();
}

fn clean_game(mut commands: Commands, entities: Query<Entity, With<GameStateEntity>>) {
    println!("Clean Game.");

    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
