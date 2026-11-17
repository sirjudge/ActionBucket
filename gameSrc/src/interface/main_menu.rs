use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

use crate::{InterfaceState, interface::menu_switcher::MenuSwapMessage};

#[derive(Component)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CurrentlySelectedOption::NewLobby)
            .add_systems(Update,update_menu.run_if(in_state(InterfaceState::InLobby)));
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedOption {
    #[default]
    NewLobby,
    JoinLobby,
    Settings,
    Exit,
}


fn update_menu(
    mut commands: Commands,
    state: ResMut<State<InterfaceState>>,
    mut next_state: ResMut<NextState<InterfaceState>>,
) {
    if state.eq(&InterfaceState::MainMenu) {
        debug!("returned early from main menu set up because the state was already main menu");
        return;
    }
    
    commands.spawn(Camera2d);
    spawn_main_menu(commands);
    next_state.set(InterfaceState::MainMenu);
}


fn spawn_main_menu(
   mut commands: Commands,
){
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
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("New Lobby button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::CreateLobby);
                        message_writer.write(message);
                    },
                ),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Join Lobby"),
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("Join Lobby button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::JoinLobby);
                        message_writer.write(message);
                    },
                ),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Settings"),
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("Settinsg button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::Settings);
                        message_writer.write(message);
                    },
                ),
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