use crate::lobby::player::{Player, PlayerPort};
use bevy::{platform::collections::HashMap, prelude::*};
use std::{fmt::Display, range};

//TODO: Reconsider this at some point in the future but I think
// a hard cap for now is a good idea to help prevent unexpected scaling problems
// when there doesn't need to be. The idea is for lobby's to not be that large so
// a cap to help keep that design in mind is I think an okay compromise.
// Only thing to think or add is a way to configure this value.
const MAX_PLAYERS_IN_GAME: i8 = 12;
const MAX_PLAYERS_IN_LOBBY: i8 = 12;

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
    pub player_list: HashMap<PlayerPort, Option<Player>>,
}

impl Display for Lobby {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let connection_limit_string = format!(
            "lobby player cap: {} in game player cap:{}",
            self.total_players_allowed_in_lobby, self.total_players_allowed_in_game
        );
        let player_list_details_string = format!(
            "Total ports initialized: {:?}",
            self.player_list.keys().count()
        );

        let mut player_list_string: String = Default::default();
        for player in self.player_list.iter() {
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
            player_list: HashMap::new(),
        }
    }

    pub fn find_first_available_port(&self) -> Option<PlayerPort> {
        for i in self.player_list.iter() {
            if i.1.is_none() {
                return Some(i.0.clone());
            }
        }
        None
    }

    pub(crate) fn add_player_to_list(&mut self, player: &Player) {
        let port = Self::find_first_available_port(self);
        match port {
            Some(port) => {
                if matches!(self.player_list.get(&port), Some(Some(_))) {
                    error!(
                        "Could not connect to Port {:?} because it's already occupied",
                        port
                    );
                    return;
                }
                self.player_list
                    .entry(port)
                    .and_modify(|player_value| {
                        *player_value = Some(player.clone());
                    });
            }
            None => {
                error!("Could not add player to port because no player ports were marked as open");
            }
        }
    }

    pub(crate) fn disconnect_player_by_player(&mut self, player: Player){
        //TODO: Need to figure out how to remove a player by their player instead of their port
        // maybe assign unique UUID or something? idk
    }

    pub(crate) fn disconnect_player_from_port(&mut self, player_port: PlayerPort) {
        self.player_list.entry(player_port).and_modify(|player| {
            *player = None;
        });
    }

    //TODO: probably bad idea to make this public but might be fine for now?
    pub(crate) fn initialize_player_list(&mut self) {
        let max_players_usize = match usize::try_from(MAX_PLAYERS_IN_LOBBY) {
            Ok(max_players) => max_players,
            Err(error) => {
                warn!(
                    "error occurred parsing max players in lobby const:{:?}",
                    error
                );
                16
            }
        };
        if self.player_list.keys().count() > 0 {
            warn!("Player list already has keys and is already initialized. Exiting init early");
            return;
        }

        if self.player_list.keys().count() > max_players_usize {
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
                    self.player_list.insert(port, None);
                }
                Err(error) => {
                    panic!("Failed parsing player port:{:?}", error);
                }
            }
        }
    }
}
