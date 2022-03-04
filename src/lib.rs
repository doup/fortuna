mod character_menu;
mod game;
mod loading;
mod lose_menu;
mod main_menu;
mod stats;
mod ui;
mod win_menu;

use bevy::prelude::*;
use character_menu::CharacterMenuPlugin;
use game::GamePlugin;
use loading::LoadingPlugin;
use lose_menu::LoseMenuPlugin;
use main_menu::MainMenuPlugin;
use win_menu::WinPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
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
            .add_plugin(CharacterMenuPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(LoseMenuPlugin)
            .add_plugin(WinPlugin);
    }
}
