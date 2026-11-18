//TODO: There's a bevy collections hashMap as well
//but tbh looks way more overhead than what I need since
//it seems to also track state or something
use crate::game::{
    fighter::{Fighter, PlayerType::Human},
    fighter_data_loader::{get_fighter_data_from_toml, load_fighter_data_to_string},
    state::GameState,
};
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque, vec_deque};

#[derive(Component)]
pub struct InGamePlugin {
    pub spawning_initialized: bool,
}

#[derive(Component)]
pub struct InGameUi;

#[derive(Resource)]
pub struct FightResource {
    fighter_list: HashMap<i16, Fighter>,
    spawn_queue: VecDeque<Fighter>,
}

impl Default for FightResource {
    fn default() -> Self {
        FightResource {
            fighter_list: HashMap::new(),
            spawn_queue: vec_deque::VecDeque::new(),
        }
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MatchState {
    #[default]
    Uninitialized,
    Initializing,
    InProgress,
    Completed,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FightResource {
            ..Default::default()
        })
        .insert_state(MatchState::Uninitialized);
        app.add_systems(Update, Self::update);
    }
}

impl InGamePlugin {
    fn update(
        game_state: Res<State<GameState>>,
        mut next_game_state: ResMut<NextState<GameState>>,
        match_state: Res<State<MatchState>>,
        mut next_match_state: ResMut<NextState<MatchState>>,
        fight_resource: ResMut<FightResource>,
        commands: Commands,
    ) {
        match game_state.get() {
            &GameState::MatchStarting => {
                //TODO: Initialize and do match starting things
                match match_state.get() {
                    MatchState::Uninitialized => {
                        //initialize
                        info!("starting match initialization");
                        Self::initialize_match(commands, fight_resource);
                        info!("completed match initialization, updating game states");
                        next_game_state.set(GameState::InMatch);
                        next_match_state.set(MatchState::InProgress);
                        info!(
                            "updated game and match state to GameState::InMatch and MatchState::InProgress"
                        );
                    }
                    _ => {
                        // TOOD: Something?
                    }
                }
            }
            _ => {
                // TOOD: Do nothing if not explicity required
            }
        }
    }

    fn initialize_match(mut commands: Commands, mut fight_resource: ResMut<FightResource>) {
        Self::initialize_fight_resource(fight_resource);
        // spawn current players in lobby
    }

    
    fn initialize_fight_resource(mut fight_resource: ResMut<FightResource>) {
        if fight_resource.fighter_list.is_empty() {
            match load_fighter_data_to_string() {
                Ok(toml_string) => {
                    let fighter_data = get_fighter_data_from_toml(&toml_string);
                    match fighter_data {
                        Ok(data) => {
                            let bucket_1_id = data.action_bucket1.character_id;
                            let bucket_1 = Fighter {
                                fighter_stats: data.action_bucket1,
                                player_type: Human,
                            };
                            debug!("Bucket 1 initialized:{:?}", bucket_1);
                            fight_resource.fighter_list.insert(bucket_1_id, bucket_1);

                            let bucket_2_id = data.action_bucket2.character_id;
                            let bucket_2 = Fighter {
                                fighter_stats: data.action_bucket2,
                                player_type: Human,
                            };
                            debug!("Bucket 2 initialized:{:?}", bucket_2);
                            fight_resource.fighter_list.insert(bucket_2_id, bucket_2);

                            let bucket_3_id = data.action_bucket3.character_id;
                            let bucket_3 = Fighter {
                                fighter_stats: data.action_bucket3,
                                player_type: Human,
                            };
                            debug!("Bucket 3 initialized:{:?}", bucket_3);
                            fight_resource.fighter_list.insert(bucket_3_id, bucket_3);

                            let bucket_4_id = data.action_bucket4.character_id;
                            let bucket_4 = Fighter {
                                fighter_stats: data.action_bucket4,
                                player_type: Human,
                            };
                            debug!("Bucket 4 initialized:{:?}", bucket_4);
                            fight_resource.fighter_list.insert(bucket_4_id, bucket_4);
                        }
                        Err(_) => {
                            error!("Error loading fight resource");
                        }
                    }
                }
                Err(_) => {
                    error!("Could not load fighter toml to a string");
                }
            }
        }
    }
}
