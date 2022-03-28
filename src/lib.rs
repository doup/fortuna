mod character_menu;
mod game;
mod loading;
mod lose_menu;
mod main_menu;
mod stats;
mod ui;
mod utils;
mod win_menu;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    CharacterMenu,
    Game,
    Loading,
    MainMenu,
    LoseMenu,
    WinMenu,
}

pub struct FortunaPlugin;

impl Plugin for FortunaPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
            .add_plugin(character_menu::CharacterMenuPlugin)
            .add_plugin(game::GamePlugin)
            .add_plugin(loading::LoadingPlugin)
            .add_plugin(main_menu::MainMenuPlugin)
            .add_plugin(lose_menu::LoseMenuPlugin)
            .add_plugin(win_menu::WinPlugin);
    }
}
