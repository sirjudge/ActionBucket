use crate::{
    SceneState,
    game::state::GameState,
    scenes::{in_game::InGamePlugin, lobby_scene::LobbyPlugin, main_menu::MainMenuPlugin},
};
use bevy::prelude::*;

pub mod debug;
pub mod in_game;
pub mod lobby_scene;
pub mod main_menu;
pub mod pallette;
pub mod splash;

#[derive(Message)]
pub struct SceneSwapMessage {
    next_state: SceneState,
}

impl SceneSwapMessage {
    pub fn new(next_state: SceneState) -> Self {
        SceneSwapMessage { next_state }
    }
}

#[derive(Component)]
pub struct MenuSwapPlugin;

impl Plugin for MenuSwapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SceneSwapMessage>()
            .add_systems(Update, swap_message_reader);
    }
}

//TODO: Need to think about having some kind of save state change here
// something like prevState: InterfaceState
// Keep blindly swapping as is for now
fn swap_message_reader(
    mut message_reader: MessageReader<SceneSwapMessage>,
    state: ResMut<State<SceneState>>,
    mut next_scene_state: ResMut<NextState<SceneState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_query: Query<&MainMenuPlugin>,
    lobby_query: Query<&LobbyPlugin>,
    in_game_query: Query<&InGamePlugin>,
    mut commands: Commands,
) {
    for message in message_reader.read() {
        // if we have the same states don't swap
        let log_message = format!("Recieved swap interface message: {:?}", message.next_state);
        info!(log_message);
        if &message.next_state == state.get() {
            return;
        }
        next_scene_state.set(message.next_state.clone());

        // handle current state despawn
        match state.get() {
            SceneState::MainMenu if menu_query.single().is_ok() => {
                commands.despawn_all::<With<MainMenuPlugin>>();
            }
            SceneState::InLobby if lobby_query.single().is_ok() => {
                commands.despawn_all::<With<LobbyPlugin>>();
            }
            SceneState::InGame if in_game_query.single().is_ok() => {
                commands.despawn_all::<With<InGamePlugin>>();
            }
            _ => (),
        }

        // spawn new stuff
        match message.next_state {
            SceneState::CreateLobby => {
                info!("Creating lobby. . .beep boop. . .");
                //TODO: Do networking things here to make lobby joinable by friends or support
                //for now just worry about spawning stuff locally
                info!("Lobby created");
                next_scene_state.set(SceneState::InLobby);
                next_game_state.set(GameState::Menu);
            }
            SceneState::MainMenu => {
                next_game_state.set(GameState::Menu);
            }
            SceneState::InGame => {
                next_game_state.set(GameState::MatchStarting);
            }
            _ => {
                debug!(
                    "Unhandled state change for next state hook:{:?}",
                    message.next_state
                );
                next_scene_state.set(message.next_state.clone());
            }
        }
    }
}
