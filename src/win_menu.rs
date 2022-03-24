use crate::loading::UIAssets;
use crate::ui::{handle_ui_buttons, NORMAL_BUTTON};
use crate::GameState;
use bevy::prelude::*;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::WinMenu).with_system(setup_win))
            .add_system_set(
                SystemSet::on_update(GameState::WinMenu)
                    .with_system(handle_ui_buttons)
                    .with_system(handle_play_again_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::WinMenu).with_system(clean_win));
    }
}

// COMPONENTS
#[derive(Component)]
struct WinMenuStateEntity;

#[derive(Component)]
struct PlayAgainButton;

// SYSTEMS
fn setup_win(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(WinMenuStateEntity);

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
        .insert(WinMenuStateEntity)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
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
                        value: String::from("You've reached the top!"),
                        style: TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    }],
                },
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(170.0), Val::Px(65.0)),
                        margin: Rect {
                            top: Val::Px(15.0),
                            right: Val::Auto,
                            bottom: Val::Px(15.0),
                            left: Val::Auto,
                        },
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PlayAgainButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play Again",
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

fn clean_win(mut commands: Commands, entities: Query<Entity, With<WinMenuStateEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_play_again_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayAgainButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            app_state.set(GameState::Game).unwrap();
        }
    }
}
