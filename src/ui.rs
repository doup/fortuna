use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn handle_ui_buttons(
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
