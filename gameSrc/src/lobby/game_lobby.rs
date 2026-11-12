use bevy::prelude::*;

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub struct Lobby {
    configuration: LobbyConfigurationOptions
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub enum GameType {
    #[default]
    Lives,
    Time,
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub struct LobbyConfigurationOptions {
    number_of_players_in_lobby: i16,
    number_of_players_allowed_in_game: i16,
    game_type: GameType
}
