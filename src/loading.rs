use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_ldtk::LdtkAsset;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<UIAssets>()
            .with_collection::<GameAssets>()
            .continue_to_state(GameState::MainMenu)
            .build(app);

        app.add_startup_system(hot_reload);
    }
}

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "goudy-bookletter-1911.otf")]
    pub font: Handle<Font>,
    #[asset(path = "logo.png")]
    pub logo: Handle<Image>,
    #[asset(path = "character.png")]
    pub character: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fortuna.ldtk")]
    pub map: Handle<LdtkAsset>,
}

fn hot_reload(_asset_server: Res<AssetServer>) {
    // asset_server.watch_for_changes().unwrap();
}
