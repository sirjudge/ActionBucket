use crate::{
    game::{
        fighter::{Fighter, PlayerType::Human},
        fighter_data_loader::{
            FighterData, get_fighter_data_from_toml, load_fighter_data_to_string,
        },
        state::GameState,
    },
    infra::rngeesus,
};
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Component)]
pub struct InGamePlugin;

#[derive(Component)]
pub struct InGameUi;

#[derive(Resource, Default)]
pub struct FightResource {
    fighter_list: HashMap<i16, Fighter>,
    spawn_queue: VecDeque<Fighter>,
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

        app.add_systems(Update, Self::state_watcher);
        app.add_systems(
            Update,
            Self::spawn_queue.run_if(in_state(GameState::InMatch)),
        );
    }
}

impl InGamePlugin {
    fn spawn_queue(
        mut fight_resource: ResMut<FightResource>,
        commands: Commands,
        asset_server: Res<AssetServer>,
        window: Single<&Window>,
    ) {
        if let Some(fighter) = fight_resource.spawn_queue.pop_front() {
            Self::spawn_fighter(fighter, commands, asset_server, window);
        }
    }

    fn spawn_fighter(
        fighter: Fighter,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window: Single<&Window>,
    ) {
        let bucket_sprite = match rngeesus::generate_rand_i8(1, 2) {
            1 => Sprite::from_image(asset_server.load("sprites/bucket_1.png")),
            2 => Sprite::from_image(asset_server.load("sprites/bucket_2.png")),
            _ => {
                //TODO: Should change this to load some kind of "sprite not found" bucket
                error!("could not load sprite so defaulting to sprite 1");
                Sprite::from_image(asset_server.load("sprites/bucket_1.png"))
            }
        };

        let x_max = window.resolution.width() / 2.0;
        let x_min = x_max;
        let y_max = window.resolution.height() / 2.0;
        let y_min = y_max;

        let rand_x = rngeesus::generate_rand_f32(x_min, x_min);
        let rand_y = rngeesus::generate_rand_f32(y_min, y_min);
        commands
            .spawn((
                Node {
                    ..Default::default()
                },
                bucket_sprite,
                fighter,
            ))
            .insert(Transform::from_xyz(rand_x, rand_y, 1.0));
    }

    //TODO: Think I can clean this method implementation up
    fn state_watcher(
        game_state: Res<State<GameState>>,
        mut next_game_state: ResMut<NextState<GameState>>,
        match_state: Res<State<MatchState>>,
        mut next_match_state: ResMut<NextState<MatchState>>,
        fight_resource: ResMut<FightResource>,
        asset_server: Res<AssetServer>,
    ) {
        match game_state.get() {
            &GameState::MatchStarting => {
                //TODO: Initialize and do match starting things
                match match_state.get() {
                    MatchState::Uninitialized => {
                        //initialize
                        info!("starting match initialization");
                        Self::initialize_match(fight_resource, asset_server);

                        info!("completed match initialization, updating game states");
                        next_game_state.set(GameState::InMatch);
                        next_match_state.set(MatchState::InProgress);
                        info!(
                            "updated game and match state to GameState::InMatch and MatchState::InProgress"
                        );
                    }
                    _ => {
                        // Do Nothing
                    }
                }
            }
            _ => {
                // Do Nothing
            }
        }
    }

    fn initialize_match(fight_resource: ResMut<FightResource>, asset_server: Res<AssetServer>) {
        Self::initialize_fight_resource(fight_resource, asset_server);
    }

    /// Takes in parsed fighter data (which contain stats and base
    /// attributes of each fighter) and the resource extraction
    fn map_fighter_data_fighter_list(data: FighterData, mut fight_resource: ResMut<FightResource>) {
        let bucket_1_id = data.action_bucket1.character_id;
        let bucket_1 = Fighter {
            fighter_stats: data.action_bucket1,
            player_type: Human,
        };
        fight_resource.spawn_queue.push_back(bucket_1);
        fight_resource.fighter_list.insert(bucket_1_id, bucket_1);

        let bucket_2_id = data.action_bucket2.character_id;
        let bucket_2 = Fighter {
            fighter_stats: data.action_bucket2,
            player_type: Human,
        };
        fight_resource.spawn_queue.push_back(bucket_1);
        fight_resource.fighter_list.insert(bucket_2_id, bucket_2);

        let bucket_3_id = data.action_bucket3.character_id;
        let bucket_3 = Fighter {
            fighter_stats: data.action_bucket3,
            player_type: Human,
        };
        fight_resource.spawn_queue.push_back(bucket_1);
        fight_resource.fighter_list.insert(bucket_3_id, bucket_3);

        let bucket_4_id = data.action_bucket4.character_id;
        let bucket_4 = Fighter {
            fighter_stats: data.action_bucket4,
            player_type: Human,
        };
        fight_resource.spawn_queue.push_back(bucket_1);
        fight_resource.fighter_list.insert(bucket_4_id, bucket_4);

        info!("Bucket 1 initialized:{:?}", bucket_1);
        info!("Bucket 2 initialized:{:?}", bucket_2);
        info!("Bucket 3 initialized:{:?}", bucket_3);
        info!("Bucket 4 initialized:{:?}", bucket_4);
    }

    fn initialize_fight_resource(
        fight_resource: ResMut<FightResource>,
        asset_server: Res<AssetServer>,
    ) {
        if fight_resource.fighter_list.is_empty() {
            match load_fighter_data_to_string(asset_server) {
                Ok(toml_string) => {
                    //TODO: Should eventually  load this smarter
                    // since we don't need every single fighter but hardcoding loading 4 buckets in
                    // is fine for now
                    let fighter_data = get_fighter_data_from_toml(&toml_string);
                    match fighter_data {
                        Ok(data) => {
                            Self::map_fighter_data_fighter_list(data, fight_resource);
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
