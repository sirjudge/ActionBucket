use crate::lobby::player::{Player, PlayerPort};
use bevy::{platform::collections::HashMap, prelude::*};
use std::fmt::Display;
use strum::IntoEnumIterator;
use uuid::Uuid;

//TODO: Reconsider this at some point in the future but I think
// a hard cap for now is a good idea to help prevent unexpected scaling problems
// when there doesn't need to be. The idea is for lobby's to not be that large so
// a cap to help keep that design in mind is I think an okay compromise.
// Only thing to think or add is a way to configure this value.
const MAX_PLAYERS_IN_GAME: i8 = 8;
const MAX_PLAYERS_IN_LOBBY: i8 = 16;

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub enum MatchEndConditionType {
    #[default]
    Lives,
    Time,
}

#[derive(Resource, Clone, Debug)]
pub struct Lobby {
    pub total_players_allowed_in_lobby: i8,
    pub total_players_allowed_in_game: i8,
    pub game_type: MatchEndConditionType,
    pub port_player_map: HashMap<PlayerPort, Option<Player>>,
    pub hosting_port: PlayerPort,
}

impl Display for Lobby {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let connection_limit_string = format!(
            "lobby player cap: {} in game player cap:{}",
            self.total_players_allowed_in_lobby, self.total_players_allowed_in_game
        );
        let player_list_details_string = format!(
            "Total ports initialized: {:?}",
            self.port_player_map.keys().count()
        );

        let mut player_list_string: String = Default::default();
        for player in self.port_player_map.iter() {
            player_list_string += &format!("{:?}, ", player);
        }

        write!(
            f,
            "{},{},{}, {}",
            connection_limit_string,
            player_list_details_string,
            stringify!(game_type),
            player_list_string
        )
    }
}

impl Lobby {
    pub(crate) fn default() -> Self {
        Self {
            total_players_allowed_in_lobby: MAX_PLAYERS_IN_LOBBY,
            total_players_allowed_in_game: MAX_PLAYERS_IN_GAME,
            game_type: MatchEndConditionType::Lives,
            port_player_map: HashMap::new(),
            //NOTE: This default setting is always assuming the first player
            //will ALWAYS be in port 1
            hosting_port: PlayerPort::One,
        }
    }

    pub fn find_first_available_port(&mut self) -> Option<PlayerPort> {
        if self.port_player_map.is_empty() {
            warn!("Player list is currently uninitialized");
            self.initialize_port_mapping();
        }

        PlayerPort::iter().find(|port| {
            self.port_player_map.contains_key(port) && self.port_player_map[port].is_none()
        })
    }

    pub(crate) fn add_player_to_list(&mut self, player: &Player) {
        let port = Self::find_first_available_port(self);
        match port {
            Some(port) => {
                if matches!(self.port_player_map.get(&port), Some(Some(_))) {
                    error!(
                        "Could not connect to Port {:?} because it's already occupied",
                        port
                    );
                    return;
                }
                self.port_player_map.entry(port).and_modify(|player_value| {
                    let mut updated_player = player.clone();
                    updated_player.player_connection_id = Some(Uuid::new_v4());
                    *player_value = Some(updated_player);
                });
            }
            None => {
                error!("Could not add player to port because no player ports were marked as open");
            }
        }
    }

    pub(crate) fn disconnect_player_with_player(
        &mut self,
        player_to_disconnect: &Player,
    ) -> Result<(), &str> {
        //TODO: Need to figure out how to remove a player by their player instead of their port
        // maybe assign unique UUID or something? idk
        match player_to_disconnect.player_connection_id {
            Some(id) => {
                info!("player has connection id Id:{}", id);

                let mut port_to_disconnect: Option<PlayerPort> = None;
                for port in PlayerPort::iter() {
                    self.port_player_map
                        .entry(port.clone())
                        .and_modify(|player_option| {
                            if let Some(player) = player_option
                                && player.eq(&player_to_disconnect)
                            {
                                port_to_disconnect = Some(port);
                            }
                        });
                    if port_to_disconnect.is_some() {
                        break;
                    }
                }
                match port_to_disconnect {
                    Some(found_port) => {
                        self.disconnect_player_with_port(&found_port);
                        Ok(())
                    }
                    None => Err("Could not find port to disconnect player from"),
                }
            }
            None => Err("Could not disconnect player because player has no player id"),
        }
    }

    pub(crate) fn disconnect_player_with_port(&mut self, player_port: &PlayerPort) {
        if player_port == &self.hosting_port {
            error!(
                "Current player is hosting the lobby and the functionality to pass along the lobby to a new player is not yet implemented"
            );
            return;
        }

        self.port_player_map
            .entry(player_port.clone())
            .and_modify(|player| {
                *player = None;
            });
    }

    pub(crate) fn initialize_port_mapping(&mut self) {
        //TODO: Need to add a unit test for this case
        if self.port_player_map.is_empty() {
            error!("Port mapping table is already initailzied and is non-empty, returning early");
            return;
        }

        let max_players_usize = usize::try_from(MAX_PLAYERS_IN_LOBBY).unwrap_or_else(|error| {
            warn!(
                "error occurred parsing max players in lobby const:{:?}",
                error
            );
            16
        });

        if self.port_player_map.keys().count() > 0 {
            warn!("Player list already has keys and is already initialized. Exiting init early");
            return;
        }

        if self.port_player_map.keys().count() > max_players_usize {
            error!(
                "Detected more players than MAX_PLAYERS_IN_LOBBY ceiling parameter. Existing init early"
            );
            return;
        }

        for i in 1..max_players_usize {
            let i8_i = match i8::try_from(i) {
                Ok(i8_int) => i8_int,
                Err(error) => {
                    panic!(
                        "Could not parse i8 from port number {} due to error:{:?}",
                        i, error
                    );
                }
            };

            match PlayerPort::i8_to_port(&i8_i) {
                Ok(port) => {
                    self.port_player_map.insert(port, None);
                }
                Err(error) => {
                    panic!("Failed parsing player port:{:?}", error);
                }
            }
        }
    }
}

#[cfg(test)]
mod game_lobby_struct_test {
    use super::*;

    #[test]
    fn lobby_default_init_successul() {
        let lobby = Lobby::default();
        assert!(lobby.total_players_allowed_in_game == MAX_PLAYERS_IN_GAME);
        assert!(lobby.total_players_allowed_in_lobby == MAX_PLAYERS_IN_LOBBY);
        assert!(lobby.game_type == MatchEndConditionType::Lives);
        assert!(lobby.port_player_map.is_empty());
    }

    #[test]
    fn lobby_initialization_method_successfully_inits_ports_with_no_players() {
        let mut lobby = Lobby::default();
        assert!(lobby.port_player_map.is_empty());
        lobby.initialize_port_mapping();
        assert!(!lobby.port_player_map.is_empty());
        assert!(lobby.port_player_map.keys().count() > 0);
    }

    #[test]
    fn find_first_available_port() {
        let mut lobby = Lobby::default();
        lobby.initialize_port_mapping();
        match lobby.find_first_available_port() {
            Some(port) => {
                assert!(port == PlayerPort::One);
            }
            None => {
                panic!("expected to find open port but returned none instead");
            }
        }
    }

    #[test]
    fn lobby_format_method_runs_without_error() {
        let lobby = Lobby::default();
        let lobby_string = format!("{:?}", lobby);
        assert!(
            !lobby_string.is_empty(),
            "expected lobby string to be a non empty string but it was"
        );
    }

    #[test]
    fn add_player_to_empty_list_sets_first_port() {
        let mut lobby = Lobby::default();
        lobby.initialize_port_mapping();
        let player = Player::default();
        lobby.add_player_to_list(&player);

        match lobby.port_player_map.get(&PlayerPort::One) {
            Some(player_option) => {
                assert!(
                    player_option.is_some(),
                    "expected player to be in port one but was not"
                );
                if let Some(player) = player_option {
                    assert!(
                        player.player_connection_id.is_some(),
                        "expected player connection id to be set but was none instead"
                    );
                }
            }
            None => {
                panic!("expected to find port 1 in hash map keys but was not found");
            }
        }
    }

    //TODO: need to add the following unit tests
    
    //adding player to full lobby
    
    //adding player to uninitialized player list
    
    //disconnecting player with ID not in player list
    
    //disconnecting player with ID in player list
    
    //disconnecting player by port

    // trying to initialize_port_mapping when already initialized returns in no error and also ends
    // early without re-setting everything
}
