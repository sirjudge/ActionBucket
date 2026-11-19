use bevy::prelude::{Resource, States};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Initializing,
    Initialized,
    Menu,
    Paused,
    Loading,
    GameOver,
    MatchStarting,
    InMatch,
}

#[derive(States, Resource, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FighterState {
    #[default]
    Idle,
    Moving,
    Attacking,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneState {
    #[default]
    Startup,
    MainMenu,
    Settings,
    JoinLobby,
    CreateLobby,
    InLobby,
    InGame,
    Loading,
}

