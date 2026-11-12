use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

pub struct MainMenuPlugin;

pub fn plugin(app: &mut App) {
    app.insert_state(CurrentlySelectedOption::NewLobby)
        .add_systems(Startup, setup);
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedOption {
    #[default]
    NewLobby,
    JoinLobby,
    Settings,
    Exit,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Button,
                Text::new("New Lobby"),
                observe(|_activate: On<Activate>| {
                    info!("New Lobby button pushed");
                    //TODO: need to despawn everything here and swap 
                    //to lobby instead
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Join Lobby"),
                observe(|_activate: On<Activate>| {
                    info!("Join Lobby button pushed");
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Settings"),
                observe(|_activate: On<Activate>| {
                    info!("Settinsg button pushed");
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Quit to Desktop"),
                observe(|_activate: On<Activate>| {
                    info!("Quit to Desktop button pushed");
                }),
                Node { ..default() },
            ));
        });
}
