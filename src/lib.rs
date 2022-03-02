mod character_menu;
mod game;
mod loading;
mod main_menu;
mod score;
mod stats;
mod ui;

use bevy::prelude::*;
use game::GamePlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use score::ScorePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Loading,
    MainMenu,
    Game,
    Score,
}

pub struct FortunaPlugin;

impl Plugin for FortunaPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(ScorePlugin);
    }
}
