use bevy::prelude::*;

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub struct Lobby {
    configuration: LobbyConfigurationOptions
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub struct LobbyConfigurationOptions {
    numberOfPlayersInLobby: i16,
    numberOfPlayersAllowedInGame: i16,
}
