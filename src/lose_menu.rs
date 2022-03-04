use crate::{loading::UIAssets, ui::NORMAL_BUTTON, GameState};
use bevy::prelude::*;

pub struct LoseMenuPlugin;

impl Plugin for LoseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::LoseMenu).with_system(setup_score))
            .add_system_set(
                SystemSet::on_update(GameState::LoseMenu).with_system(handle_restart_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::LoseMenu).with_system(clean_score));
    }
}

// COMPONENTS
#[derive(Component)]
struct LoseMenuStateEntity;

#[derive(Component)]
struct RetryButton;

// SYSTEMS
fn setup_score(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(LoseMenuStateEntity);

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
        .insert(LoseMenuStateEntity)
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
                            value: String::from("You lose, jump harder next time."),
                            style: TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        }],
                    },
                    ..Default::default()
                })
                .insert(LoseMenuStateEntity);

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
                .insert(RetryButton)
                .insert(LoseMenuStateEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Retry",
                                TextStyle {
                                    font: ui_assets.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(LoseMenuStateEntity);
                });
        });
}

fn handle_restart_button(
    mut app_state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RetryButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            app_state.set(GameState::Game).unwrap();
        }
    }
}

fn clean_score(mut commands: Commands, entities: Query<Entity, With<LoseMenuStateEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
