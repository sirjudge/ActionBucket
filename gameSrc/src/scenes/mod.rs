use bevy::prelude::*;
use crate::{InterfaceState, scenes::main_menu::MainMenuPlugin};

pub mod main_menu;
pub mod in_game;
pub mod lobby;
pub mod pallette;
pub mod splash;
pub mod debug;


#[derive(Message)]
pub struct MenuSwapMessage {
    next_state: InterfaceState,
}

impl MenuSwapMessage {
    pub fn new(next_state: InterfaceState) -> Self {
        MenuSwapMessage { next_state }
    }
}

#[derive(Component)]
pub struct MenuSwapPlugin;

impl Plugin for MenuSwapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MenuSwapMessage>()
            .add_systems(Update, swap_message_reader);
    }
}

//TODO: Need to think about having some kind of save state change here 
// something like prevState: InterfaceState
// Keep blindly swapping as is for now
fn swap_message_reader(
    mut message_reader: MessageReader<MenuSwapMessage>,
    state: ResMut<State<InterfaceState>>,
    mut next_state: ResMut<NextState<InterfaceState>>,
    menu_query: Query<&MainMenuPlugin>,
    mut commands: Commands,
) {
    for message in message_reader.read() {
        // if we have the same states don't swap
        let log_message = format!("Recieved swap interface message: {:?}", message.next_state);
        info!(log_message);
        if &message.next_state == state.get() {
            return;
        }
        next_state.set(message.next_state.clone());

        match state.get() {
            InterfaceState::Startup => todo!(),
            InterfaceState::MainMenu => {
                // if the current interface is lobby + we can get the root menu entity despawn
                if state.eq(&InterfaceState::MainMenu)
                    && let Ok(_) = menu_query.single()
                {
                    commands.despawn_all::<With<MainMenuPlugin>>();
                }
            },
            InterfaceState::Settings => todo!(),
            InterfaceState::JoinLobby => todo!(),
            InterfaceState::CreateLobby => todo!(),
            InterfaceState::InLobby => todo!(),
            InterfaceState::InGame => todo!(),
            InterfaceState::Loading => todo!(),
        }

        match message.next_state {
            InterfaceState::Startup => todo!(),
            InterfaceState::MainMenu => {
                match menu_query.single(){
                    Ok(_) => {
                        info!("successfully matched main menu query when switching menus");
                    },
                    Err(_) => {
                        error!("Main menu query failed in menu swithcing");
                    },
                }
            },
            InterfaceState::Settings => todo!(),
            InterfaceState::JoinLobby => todo!(),
            InterfaceState::CreateLobby =>{
                info!("Creating lobby. . .beep boop. . .");
                //TODO: Do networking things here to make lobby joinable by friends or support
                info!("Lobby created");
                next_state.set(InterfaceState::InLobby);

            },
            InterfaceState::InLobby => todo!(),
            InterfaceState::InGame => todo!(),
            InterfaceState::Loading => todo!(),
        }
    }
}
