use crate::loading::UIAssets;
use crate::ui::{handle_ui_buttons, NORMAL_BUTTON};
use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(handle_ui_buttons)
                    .with_system(handle_play_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(clean_main_menu));
    }
}

// COMPONENTS
#[derive(Component)]
struct MainMenuStateEntity;

#[derive(Component)]
struct PlayButton;

// SYSTEMS
fn setup_main_menu(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MainMenuStateEntity);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: Rect {
                    top: Val::Auto,
                    right: Val::Px(0.0),
                    bottom: Val::Auto,
                    left: Val::Px(0.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(MainMenuStateEntity)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(120.0)),
                    ..Default::default()
                },
                image: ui_assets.logo.clone().into(),
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: Rect {
                            top: Val::Px(15.0),
                            right: Val::Undefined,
                            bottom: Val::Undefined,
                            left: Val::Undefined,
                        },
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PlayButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
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
                    });
                });
        });
}

fn clean_main_menu(mut commands: Commands, entities: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_play_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            app_state.set(GameState::Game).unwrap();
        }
    }
}
