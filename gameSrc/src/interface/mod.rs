use bevy::prelude::*;

pub mod main_menu;
pub mod fight_ui;
pub mod lobby_menu;
pub mod pallette;


#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum MenuTypes {
    #[default]
    None,
    Main,
    Settings,
    InGamePause,
    InLobby,
    JoinLobby,
}
