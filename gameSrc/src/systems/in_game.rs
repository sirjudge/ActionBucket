use crate::{
    game::{
        fighter::Fighter,
        fighter_data_loader::{get_fighter_data_from_toml, load_fighter_data_to_string},
        state::GameState,
    },
    infra::rngeesus,
    lobby::game_lobby::Lobby,
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

impl FightResource {
    fn get_random_fighter_from_list(&self) -> Fighter {
        if self.fighter_list.is_empty() {
            panic!("Error spawning fighter's fighter list is empty");
        }

        let random_id = rngeesus::generate_rand_i16(1, 4);
        match self.fighter_list.get(&random_id) {
            Some(fighter) => *fighter,
            None => {
                panic!("Could not find chosen fighter in fighter list")
            }
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

        let spawn_coords = Self::generate_spawn_coords(window);
        commands
            .spawn((
                Node {
                    ..Default::default()
                },
                bucket_sprite,
                fighter,
            ))
            .insert(Transform::from_xyz(
                spawn_coords.x,
                spawn_coords.y,
                spawn_coords.z,
            ));
    }

    fn generate_spawn_coords(window: Single<&Window>) -> Vec3 {
        let x_max = window.resolution.width() / 4.0;
        let x_min = -x_max;
        let y_max = window.resolution.height() / 4.0;
        let y_min = -y_max;

        let rand_x = rand::random_range(x_min..x_max);
        let rand_y = rand::random_range(y_min..y_max);

        debug!("x_max:{:?} x_min {:?}", x_max, x_min);
        debug!("y_max:{:?} y_max: {:?}", y_max, y_min);
        debug!("Spawning fighter at ({:?},{:?})", rand_x, rand_y);
        vec3(rand_x, rand_y, 0.0)
    }

    fn state_watcher(
        game_state: Res<State<GameState>>,
        mut next_game_state: ResMut<NextState<GameState>>,
        match_state: Res<State<MatchState>>,
        mut next_match_state: ResMut<NextState<MatchState>>,
        fight_resource: ResMut<FightResource>,
        asset_server: Res<AssetServer>,
        lobby: ResMut<Lobby>,
    ) {
        match game_state.get() {
            &GameState::MatchStarting => {
                //TODO: Initialize and do match starting things
                match match_state.get() {
                    MatchState::Uninitialized => {
                        //initialize
                        info!("starting match initialization");
                        Self::initialize_match(fight_resource, asset_server, lobby);

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

    fn initialize_match(
        mut fight_resource: ResMut<FightResource>,
        asset_server: Res<AssetServer>,
        lobby: ResMut<Lobby>,
    ) {
        //Initialize fighter list for which to spawn fighters in from
        if fight_resource.fighter_list.is_empty() {
            match load_fighter_data_to_string(asset_server) {
                Ok(toml_string) => {
                    //TODO: Should eventually  load this smarter
                    // since we don't need every single fighter but hardcoding loading 4 buckets in
                    // is fine for now
                    let fighter_data = get_fighter_data_from_toml(&toml_string);
                    match fighter_data {
                        Ok(data) => {
                            let bucket_1_id = data.action_bucket1.character_id;
                            let bucket_1 = Fighter {
                                fighter_stats: data.action_bucket1,
                            };
                            fight_resource.fighter_list.insert(bucket_1_id, bucket_1);

                            let bucket_2_id = data.action_bucket2.character_id;
                            let bucket_2 = Fighter {
                                fighter_stats: data.action_bucket2,
                            };
                            fight_resource.fighter_list.insert(bucket_2_id, bucket_2);

                            let bucket_3_id = data.action_bucket3.character_id;
                            let bucket_3 = Fighter {
                                fighter_stats: data.action_bucket3,
                            };
                            fight_resource.fighter_list.insert(bucket_3_id, bucket_3);

                            let bucket_4_id = data.action_bucket4.character_id;
                            let bucket_4 = Fighter {
                                fighter_stats: data.action_bucket4,
                            };
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

        // initialize and spawn in each player's fighters
        for player in lobby.port_player_map.iter() {
            match player.1 {
                None => (),
                Some(player) => match player.selected_fighter {
                    None => {
                        warn!(
                            "Player currently loaded in has no selected fighter:{:?}, defaulting to random instead",
                            player
                        );
                        //TODO: need to spawn random fighter
                        // fight_resource.spawn_queue.push_back(value);
                    }
                    Some(fighter) => {
                        info!(
                            "spawning player:{:?} with selected fighter of :{:?}",
                            player, fighter
                        );
                        fight_resource.spawn_queue.push_back(fighter);
                    }
                },
            }
        }
    }
}
