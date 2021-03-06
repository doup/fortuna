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
    // Badges
    #[asset(path = "badge-skin-light.png")]
    pub badge_skin_light: Handle<Image>,
    #[asset(path = "badge-skin-medium.png")]
    pub badge_skin_medium: Handle<Image>,
    #[asset(path = "badge-skin-dark.png")]
    pub badge_skin_dark: Handle<Image>,
    #[asset(path = "badge-male.png")]
    pub badge_male: Handle<Image>,
    #[asset(path = "badge-female.png")]
    pub badge_female: Handle<Image>,
    #[asset(path = "badge-rich.png")]
    pub badge_rich: Handle<Image>,
    #[asset(path = "badge-smart.png")]
    pub badge_smart: Handle<Image>,
    #[asset(path = "badge-strong.png")]
    pub badge_strong: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fortuna.ldtk")]
    pub map: Handle<LdtkAsset>,
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 35, rows = 1))]
    #[asset(path = "player.png")]
    pub player_atlas: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 32.0, columns = 37, rows = 1))]
    #[asset(path = "vfx.png")]
    pub vfx_atlas: Handle<TextureAtlas>,
    #[asset(path = "jump.ogg")]
    pub jump_sound: Handle<AudioSource>,
}

fn hot_reload(_asset_server: Res<AssetServer>) {
    // asset_server.watch_for_changes().unwrap();
}
