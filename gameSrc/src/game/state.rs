use bevy::prelude::{Resource, States};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputType {
    Keyboard,
    Controller,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Initializing,
    MainMenu,
    Dialogue,
    Paused,
    Loading,
    Saving,
    GameOver,
    TearDown,
    MatchStarting,
    InMatch,
}

#[derive(States, Resource, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Attacking,
}

