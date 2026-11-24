use crate::game::fighter::Fighter;
use bevy::{asset::uuid::Uuid, log::error, state::state::States};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Debug, Default, States, Eq, PartialEq, Hash, Clone, EnumIter)]
#[repr(i8)]
pub enum PlayerPort {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
}

impl PlayerPort {
    pub fn i8_to_port(port_number: &i8) -> Result<PlayerPort, &str> {
        if *port_number <= 0 {
            let return_string = stringify!(
                "Could not parse port number less than or equal to 0:{}",
                port_number.clone()
            );
            error!(return_string);
            return Err(return_string);
        }
        match port_number {
            1 => Ok(PlayerPort::One),
            2 => Ok(PlayerPort::Two),
            3 => Ok(PlayerPort::Three),
            4 => Ok(PlayerPort::Four),
            5 => Ok(PlayerPort::Five),
            6 => Ok(PlayerPort::Six),
            7 => Ok(PlayerPort::Seven),
            8 => Ok(PlayerPort::Eight),
            9 => Ok(PlayerPort::Nine),
            10 => Ok(PlayerPort::Ten),
            11 => Ok(PlayerPort::Eleven),
            12 => Ok(PlayerPort::Twelve),
            13 => Ok(PlayerPort::Thirteen),
            14 => Ok(PlayerPort::Fourteen),
            15 => Ok(PlayerPort::Fifteen),
            16 => Ok(PlayerPort::Sixteen),
            _ => Err(stringify!(
                "Player Port numbers above 16 aren't allowed:{:?}",
                port_number
            )),
        }
    }
}

#[derive(Debug, Default, States, Eq, PartialEq, Hash, Clone)]
pub enum InputType {
    #[default]
    Keyboard,
    Controller,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum PlayerConnectionType {
    #[default]
    Unconnected,
    Local,
    Remote,
}

#[derive(Default, Clone, Debug)]
pub struct Player {
    pub selected_fighter: Option<Fighter>,
    pub player_connection_type: PlayerConnectionType,
    pub player_connection_id: Option<Uuid>,
}

//TODO: Should come back and re-evaluate this later
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        (self.player_connection_id == other.player_connection_id)
            && (self.player_connection_type == other.player_connection_type)
    }
}

// Eq is a marker trait. The empty body asserts that the custom
// PartialEq logic is reflexive, symmetric, and transitive.
impl Eq for Player {}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let connection_string = match self.player_connection_type {
            PlayerConnectionType::Unconnected => "Unconnected Player".to_string(),
            PlayerConnectionType::Local => "Connected locally".to_string(),
            PlayerConnectionType::Remote => "Connected Remote".to_string(),
        };

        let fighter_string = match self.selected_fighter {
            Some(fighter) => {
                format!("Fighter id:{:?}", fighter.fighter_stats.character_id).to_string()
            }
            None => "No fighter selected".to_string(),
        };

        write!(f, "{},{}", connection_string, fighter_string)
    }
}
