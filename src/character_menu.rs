use crate::loading::UIAssets;
use crate::stats::{Stats, StatsRes};
use crate::ui::{handle_ui_buttons, NORMAL_BUTTON};
use crate::GameState;
use bevy::prelude::*;

pub struct CharacterMenuPlugin;

impl Plugin for CharacterMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_character_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(handle_ui_buttons)
                    .with_system(handle_start_button)
                    .with_system(handle_reborn_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_character_menu));
    }
}

// COMPONENTS
#[derive(Component)]
struct CharacterMenuStateEntity;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ReBornButton;

#[derive(Component)]
struct StatsDescription;

// SYSTEMS
fn setup_character_menu(stats: Res<StatsRes>, mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CharacterMenuStateEntity);

    let button_margin = Rect {
        top: Val::Px(15.0),
        right: Val::Auto,
        bottom: Val::Px(15.0),
        left: Val::Auto,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: Rect::all(Val::Px(0.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(CharacterMenuStateEntity)
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(100.0)),
                        ..Default::default()
                    },
                    image: ui_assets.character.clone().into(),
                    ..Default::default()
                })
                .insert(CharacterMenuStateEntity);

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(600.0), Val::Px(140.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CharacterMenuStateEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                max_size: Size::new(Val::Px(600.0), Val::Auto),
                                ..Default::default()
                            },
                            // Use `Text` directly
                            text: Text {
                                alignment: TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    vertical: VerticalAlign::Center,
                                },
                                sections: vec![TextSection {
                                    value: stats.0.get_description(),
                                    style: TextStyle {
                                        font: ui_assets.font.clone(),
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                }],
                            },
                            ..Default::default()
                        })
                        .insert(CharacterMenuStateEntity)
                        .insert(StatsDescription);
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: button_margin,
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(ReBornButton)
                .insert(CharacterMenuStateEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Re-Born",
                                TextStyle {
                                    font: ui_assets.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(CharacterMenuStateEntity);
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: button_margin,
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(StartButton)
                .insert(CharacterMenuStateEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Start",
                                TextStyle {
                                    font: ui_assets.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(CharacterMenuStateEntity);
                });
        });
}

fn clean_character_menu(
    mut commands: Commands,
    entities: Query<Entity, With<CharacterMenuStateEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_start_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("Play clicked");
            // app_state.set(GameState::Game).unwrap();
        }
    }
}

fn handle_reborn_button(
    mut stats: ResMut<StatsRes>,
    mut stats_desc_query: Query<&mut Text, With<StatsDescription>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReBornButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            stats.0 = Stats::new();
            let mut stats_desc = stats_desc_query.single_mut();
            stats_desc.sections[0].value = stats.0.get_description();
        }
    }
}
