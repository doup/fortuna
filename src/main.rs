use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

fn main() {
    let mut app = App::new();

    // Add asset loader
    AssetLoader::new(GameState::Loading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<UIAssets>()
        // .with_collection::<ImageAssets>()
        // .with_collection::<AudioAssets>()
        .build(&mut app);

    app
        // RESOURCES
        .insert_resource(WindowDescriptor {
            title: "Fortuna".to_string(),
            ..Default::default()
        })
        // PLUGINS
        .add_plugins(DefaultPlugins)
        // GENERAL
        // .add_startup_system(setup_camera)
        .add_state(GameState::Loading) // Initial State
        // MAIN MENU
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(handle_ui_buttons)
                .with_system(handle_play_button)
                .with_system(handle_reroll_button),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_main_menu))
        // GAME
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game))
        // RESULT
        .add_system_set(SystemSet::on_enter(GameState::Result).with_system(setup_result))
        .add_system_set(
            SystemSet::on_update(GameState::Result)
                .with_system(handle_ui_buttons)
                .with_system(handle_restart_button),
        )
        .add_system_set(SystemSet::on_exit(GameState::Result).with_system(close_result))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Loading,
    MainMenu,
    Game,
    Result,
}

// Resources
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(AssetCollection)]
struct UIAssets {
    #[asset(path = "GoudyBookletter1911.otf")]
    font: Handle<Font>,
}

// COMPONENTS
#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct ResultUI;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ReRollButton;

#[derive(Component)]
struct RestartButton;

// SYSTEMS
// fn setup_camera(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
// }

fn setup_main_menu(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MainMenuUI);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),            // center button
                justify_content: JustifyContent::Center, // horizontally center child text
                align_items: AlignItems::Center,         // vertically center child text
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(PlayButton)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Play",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(MainMenuUI);
        });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),            // center button
                justify_content: JustifyContent::Center, // horizontally center child text
                align_items: AlignItems::Center,         // vertically center child text
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ReRollButton)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Re-Roll",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(MainMenuUI);
        });
}

fn close_main_menu(mut commands: Commands, buttons: Query<Entity, With<MainMenuUI>>) {
    for entity in buttons.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn handle_play_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("Play clicked");
            app_state.set(GameState::Game).unwrap();
        }
    }
}

fn handle_reroll_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReRollButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("ReRoll clicked");
        }
    }
}

// GAME
fn setup_game(mut app_state: ResMut<State<GameState>>) {
    println!("Game started… aaaand finished. Moving to Result state.");
    app_state.set(GameState::Result).unwrap();
}

// RESULT
fn setup_result(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ResultUI);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),            // center button
                justify_content: JustifyContent::Center, // horizontally center child text
                align_items: AlignItems::Center,         // vertically center child text
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(RestartButton)
        .insert(ResultUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Restart",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(ResultUI);
        });
}

fn handle_restart_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            app_state.set(GameState::MainMenu).unwrap();
        }
    }
}

fn close_result(mut commands: Commands, buttons: Query<Entity, With<ResultUI>>) {
    for entity in buttons.iter() {
        commands.entity(entity).despawn();
    }
}
