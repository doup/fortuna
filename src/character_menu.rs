use crate::game::{LifesText, Player, PlayerPositionsRes, Position};
use crate::loading::UIAssets;
use crate::stats::{Intelligence, SkinColor, Stats, StatsRes, Strength, Wealth};
use crate::ui::{handle_ui_buttons, NORMAL_BUTTON};
use crate::GameState;
use bevy::prelude::*;

pub struct CharacterMenuPlugin;

impl Plugin for CharacterMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::CharacterMenu).with_system(setup_character_menu),
        )
        .add_system_set(
            SystemSet::on_update(GameState::CharacterMenu)
                .with_system(handle_ui_buttons)
                .with_system(handle_start_button)
                .with_system(handle_reborn_button),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::CharacterMenu).with_system(clean_character_menu),
        );
    }
}

// COMPONENTS
#[derive(Component)]
struct CharacterMenuStateEntity;

#[derive(Component)]
struct BadgesNode;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ReBornButton;

#[derive(Component)]
struct StatsDescription;

// SYSTEMS
fn setup_character_menu(stats: Res<StatsRes>, mut commands: Commands, ui_assets: Res<UIAssets>) {
    let button_margin = Rect::all(Val::Px(15.0));

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
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(100.0)),
                    ..Default::default()
                },
                image: ui_assets.character.clone().into(),
                ..Default::default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(100.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BadgesNode)
                .with_children(|parent| {
                    add_badges(parent, &stats.value, &ui_assets);
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(600.0), Val::Px(220.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
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
                                    value: stats.value.get_description(),
                                    style: TextStyle {
                                        font: ui_assets.font.clone(),
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                }],
                            },
                            ..Default::default()
                        })
                        .insert(StatsDescription);

                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            max_size: Size::new(Val::Px(600.0), Val::Auto),
                            margin: Rect {
                                top: Val::Px(15.0),
                                right: Val::Undefined,
                                bottom: Val::Undefined,
                                left: Val::Undefined,
                            },
                            ..Default::default()
                        },
                        // Use `Text` directly
                        text: Text {
                            alignment: TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                            sections: vec![TextSection {
                                value: String::from("Jump hard and reach to the top!"),
                                style: TextStyle {
                                    font: ui_assets.font.clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                            }],
                        },
                        ..Default::default()
                    });

                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            max_size: Size::new(Val::Px(600.0), Val::Auto),
                            margin: Rect {
                                top: Val::Px(10.0),
                                right: Val::Undefined,
                                bottom: Val::Undefined,
                                left: Val::Undefined,
                            },
                            ..Default::default()
                        },
                        // Use `Text` directly
                        text: Text {
                            alignment: TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                            sections: vec![TextSection {
                                value: String::from("Oh! And don't let the black goo catch you..."),
                                style: TextStyle {
                                    font: ui_assets.font.clone(),
                                    font_size: 28.0,
                                    color: Color::BLACK,
                                },
                            }],
                        },
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        margin: Rect::all(Val::Px(30.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::WHITE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                margin: button_margin,
                                justify_content: JustifyContent::Center, // horizontally center child text
                                align_items: AlignItems::Center, // vertically center child text
                                ..Default::default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..Default::default()
                        })
                        .insert(ReBornButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
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
                            });
                        });

                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                margin: button_margin,
                                justify_content: JustifyContent::Center, // horizontally center child text
                                align_items: AlignItems::Center, // vertically center child text
                                ..Default::default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..Default::default()
                        })
                        .insert(StartButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
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
                            });
                        });
                });
        });
}

fn clean_character_menu(
    mut commands: Commands,
    entities: Query<Entity, With<CharacterMenuStateEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn add_badges(parent: &mut ChildBuilder, stats: &Stats, ui_assets: &UIAssets) {
    let mut badges: Vec<Handle<Image>> = vec![];

    match stats.color {
        SkinColor::Light => badges.push(ui_assets.badge_skin_light.clone()),
        SkinColor::Medium => badges.push(ui_assets.badge_skin_medium.clone()),
        SkinColor::Dark => badges.push(ui_assets.badge_skin_dark.clone()),
    }

    match stats.is_male {
        true => badges.push(ui_assets.badge_male.clone()),
        false => badges.push(ui_assets.badge_female.clone()),
    }

    if stats.wealth == Wealth::Rich {
        badges.push(ui_assets.badge_rich.clone());
    }

    if stats.strength == Strength::Strong {
        badges.push(ui_assets.badge_strong.clone());
    }

    if stats.intelligence == Intelligence::Smart {
        badges.push(ui_assets.badge_smart.clone());
    }

    for image in badges {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                ..Default::default()
            },
            image: image.into(),
            ..Default::default()
        });
    }
}

fn handle_start_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            // Exit from `character_menu`
            app_state.pop().unwrap();
        }
    }
}

fn handle_reborn_button(
    ui_assets: Res<UIAssets>,
    mut commands: Commands,
    mut stats: ResMut<StatsRes>,
    mut stats_desc_query: Query<&mut Text, (With<StatsDescription>, Without<LifesText>)>,
    mut player_query: Query<&mut Position, With<Player>>,
    mut lifes_query: Query<&mut Text, (With<LifesText>, Without<StatsDescription>)>,
    player_positions: Res<PlayerPositionsRes>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReBornButton>)>,
    badges_query: Query<Entity, With<BadgesNode>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            stats.value = Stats::new();

            let mut stats_desc = stats_desc_query.single_mut();
            stats_desc.sections[0].value = stats.value.get_description();

            // Re-create badges
            let badges_node = badges_query.single();
            commands.entity(badges_node).despawn_descendants();
            commands
                .entity(badges_node)
                .with_children(|parent| add_badges(parent, &stats.value, &ui_assets));

            // Move player
            let pos = match stats.value.wealth {
                Wealth::Rich => 0,
                Wealth::MiddleClass => 1,
                Wealth::Poor => 2,
            };

            let &position_transform = player_positions.value.get(pos).unwrap();
            let mut player_position = player_query.single_mut();

            player_position.value = position_transform.translation.truncate();

            // Update lifes
            let mut lifes_text = lifes_query.single_mut();
            lifes_text.sections[1].value = stats.value.lifes.to_string();
        }
    }
}
