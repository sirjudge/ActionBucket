use bevy::prelude::*;


#[derive(Default, Component, Clone, Debug, PartialEq)]
pub enum GameType {
    #[default]
    Lives,
    Time,
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub struct Lobby {
    total_players_allowed_in_lobby: i16,
    number_of_players_allowed_in_game: i16,
    game_type: GameType
}

impl Lobby {
    pub(crate) fn default() -> Self {
        Self { 
            total_players_allowed_in_lobby: 0,
            number_of_players_allowed_in_game: 16,
            game_type:GameType::Lives,
        }
    }

    pub(crate) fn new(total_players_allowed_in_lobby:i16,number_of_players_allowed_in_game:i16, game_type: GameType) -> Self {
        Self { 
            total_players_allowed_in_lobby,
            number_of_players_allowed_in_game,
            game_type,
        }
    }
}
