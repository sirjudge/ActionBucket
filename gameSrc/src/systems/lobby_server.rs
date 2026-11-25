use bevy::prelude::*;

use crate::{
    game::{fighter::Fighter, state::SceneState},
    lobby::{
        game_lobby::Lobby,
        player::{Player, PlayerPort},
    },
};

pub enum ConnectionMessageType {
    Connect,
    Disconnect,
}

#[derive(Message)]
pub struct PlayerCharacterChangeMessage {
    player_port: PlayerPort,
    fighter: Fighter,
}
#[derive(Message)]
pub struct PlayerConnectionMessage {
    player: Player,
    connection_message_type: ConnectionMessageType,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerState {
    Stopped,
    Initializing,
    Started,
    Stopping,
}

#[derive(Message)]
pub struct ServerStateMessage {
    pub next_state: ServerState,
}

//TODO: Need to come back and start filling this in. The lobby scene should be responsible for
//handling player client side interactions and this server will serve as the actual behind the
//scenes brain for the hosting and port management and all that jazz
// the server should have support for both remote AND local players within the same server so
// multiple people can play on the same computer or everyone can be on their own machine
#[derive(Component)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<Lobby>(Lobby::default())
            .add_message::<ServerStateMessage>()
            .insert_state(ServerState::Stopped);
        app.add_message::<PlayerConnectionMessage>()
            .add_message::<PlayerCharacterChangeMessage>();

        // app.add_systems(PostStartup, Self::init_lobby_post_startup);

        app.add_systems(
            Update,
            (
                Self::player_event_reader,
                Self::player_character_change_reader,
            )
                .run_if(in_state(SceneState::InLobby)),
        );

        app.add_systems(Update, Self::server_state_chagne_message_reader);
    }
}

impl ServerPlugin {
    fn server_state_chagne_message_reader(
        mut state_reader: MessageReader<ServerStateMessage>,
        current_server_state: Res<State<ServerState>>,
        mut next_server_state: ResMut<NextState<ServerState>>,
        mut lobby_resource: ResMut<Lobby>,
    ) {
        for message in state_reader.read() {
            if current_server_state.eq(&message.next_state) {
                return;
            }

            match message.next_state {
                ServerState::Initializing => {
                    lobby_resource.initialize_port_mapping();
                }
                ServerState::Stopping => {
                }
                // ServerState::Started => todo!(),
                // ServerState::Stopped => todo!(),
                _ => todo!()
            }

            next_server_state.set(message.next_state);
        }
    }

    fn player_event_reader(
        mut connection_message_reader: MessageReader<PlayerConnectionMessage>,
        mut lobby: ResMut<Lobby>,
    ) {
        for message in connection_message_reader.read() {
            match message.connection_message_type {
                ConnectionMessageType::Connect => {
                    info!(
                        "Processing player connection for player:{:?}",
                        message.player
                    );
                    lobby.add_player_to_list(&message.player);
                }
                ConnectionMessageType::Disconnect => {
                    info!(
                        "Processing player disconnection for player:{:?}",
                        message.player
                    );
                    match lobby.disconnect_player_by_player(&message.player) {
                        Ok(_) => {
                            info!("Successfully disconnected player from lobby");
                        }
                        Err(error) => {
                            error!("failed disconnecting player from lobby:{:?}", error);
                        }
                    }
                }
            }
        }
    }

    fn player_character_change_reader(
        mut lobby: ResMut<Lobby>,
        mut player_change_message_reader: MessageReader<PlayerCharacterChangeMessage>,
    ) {
        for message in player_change_message_reader.read() {
            lobby
                .port_player_map
                .entry(message.player_port.clone())
                .and_modify(|player_option| match player_option {
                    Some(player) => {
                        player.selected_fighter = Some(message.fighter);
                    }
                    None => {
                        error!(
                            "Could not map Player from player port:{:?}",
                            message.player_port.clone()
                        );
                    }
                });
        }
    }
}
