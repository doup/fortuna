use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<UIAssets>()
            .continue_to_state(GameState::MainMenu)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "goudy-bookletter-1911.otf")]
    pub font: Handle<Font>,
}
