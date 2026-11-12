use crate::{interface::pallette, lobby};
use bevy::{
    input_focus::tab_navigation::TabIndex, picking::hover::Hovered, prelude::*, ui_widgets::{Activate, Button, MenuItem, observe, popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide}},
};

pub fn plugin(app: &mut App) {
    let default_lobby_configuration = lobby::game_lobby::Lobby::default();
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GlobalZIndex(100),
            Popover {
                positions: vec![
                    PopoverPlacement {
                        side: PopoverSide::Bottom,
                        align: PopoverAlign::Start,
                        gap: 2.0,
                    },
                    PopoverPlacement {
                        side: PopoverSide::Top,
                        align: PopoverAlign::Start,
                        gap: 2.0,
                    },
                ],
                window_margin: 10.0,
            },
            OverrideClip
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::axes(px(8), px(2)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                children![fighter_select_item(), fighter_select_item()],
            ));

            parent.spawn((
                Button,
                Text::new("Start Match"),
                observe(|_activate: On<Activate>| {
                    info!("New Lobby button pushed");
                }),
                Node { ..default() },
            ));
        });
}

fn fighter_select_item() -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(px(8), px(2)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        MenuItem,
        Hovered::default(),
        TabIndex(0),
        BackgroundColor(pallette::BUTTON_BACKGROUND),
        children![(
            Text::new("Menu Item"),
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
