use bevy::prelude::*;

use crate::game::{
    fighter::{Fighter, PlayerType},
    state::GameState,
};

#[derive(Component)]
pub struct InGamePlugin;

#[derive(Component)]
pub struct InGameUi;

#[derive(Resource)]
pub struct FightConfiguration {
    fighter_list: Vec<Fighter>,
}

impl Default for FightConfiguration {
    fn default() -> Self {
        FightConfiguration {
            fighter_list: vec![],
        }
    }
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FightConfiguration {
            fighter_list: Vec::new(),
        });

        app.add_systems(Update, Self::update);
    }
}

impl InGamePlugin {
    fn update(game_state: Res<State<GameState>>, mut commands: Commands) {
        if game_state.get().ne(&GameState::Running) {
            // bail early if not in running state
            return;
        }
    }

    fn spawn_fighter_message_reader() {
        let fighter = Fighter {
            health_percent: 0,
            lives_remaining: 3,
            player_type: PlayerType::CPU,
        };
    }
}
