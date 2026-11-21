use bevy::state::state::States;

#[derive(Debug, Default, States, Eq, PartialEq, Hash, Clone)]
pub enum PlayerPort {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

#[derive(Debug, Default, States, Eq, PartialEq, Hash, Clone)]
pub enum InputType {
    #[default]
    Keyboard,
    Controller
}
