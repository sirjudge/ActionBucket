use crate::systems::palette::{self};
use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};

pub(crate) fn spawn_button(button_text: &str) -> impl Bundle {
    (
        Node {
            border: UiRect::all(Val::Px(2.)),
            border_radius: BorderRadius::new(
                Val::Px(20.),
                Val::Px(20.),
                Val::Px(20.),
                Val::Px(20.),
            ),
            ..default()
        },
        Button,
        Text::new(button_text),
        Hovered::default(),
        TextColor(palette::BUTTON_TEXT),
        BackgroundColor(palette::BUTTON_BACKGROUND),
        BorderColor {
            top: palette::BORDER_COLOR,
            bottom: palette::BORDER_COLOR,
            left: palette::BORDER_COLOR,
            right: palette::BORDER_COLOR,
        },
    )
}
