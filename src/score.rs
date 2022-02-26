use crate::{loading::UIAssets, ui::NORMAL_BUTTON, GameState};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Score).with_system(setup_score))
            .add_system_set(
                SystemSet::on_update(GameState::Score).with_system(handle_restart_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::Score).with_system(clean_score));
    }
}

// COMPONENTS
#[derive(Component)]
struct ScoreStateEntity;

#[derive(Component)]
struct RestartButton;

// SYSTEMS
fn setup_score(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ScoreStateEntity);

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
        .insert(ScoreStateEntity)
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
                .insert(ScoreStateEntity);
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

fn clean_score(mut commands: Commands, entities: Query<Entity, With<ScoreStateEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
