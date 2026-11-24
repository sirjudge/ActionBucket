use bevy::prelude::*;

use crate::{
    game::state::{GameState, SceneState}, scenes::{
        MenuSwapPlugin, debug::DebugPlugin, in_game::InGamePlugin, lobby_scene::LobbyPlugin, main_menu::MainMenuPlugin,
    },
};

pub mod game;
pub mod infra;
pub mod lobby;
pub mod scenes;
pub mod tracking;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "Action Bucket".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );

        app.insert_state(SceneState::MainMenu)
            .insert_state(GameState::Initializing);

        app.add_plugins((
            infra::audio::plugin,
            LobbyPlugin,
            MainMenuPlugin,
            MenuSwapPlugin,
            InGamePlugin,
            DebugPlugin,
        ));

        app.configure_sets(Update, (AppSystems::Update,).chain());

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera.spawn());

        app.add_systems(PostStartup, initialization_complete);
    }
}

fn initialization_complete(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Initialized);
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    //TODO: These came with the bevy tutorial so don't need for now
    // they look interesting and tickTimers and recordInput sound
    // like things I want to tinker with
    // TickTimers,
    // RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera() -> impl Scene {
    bsn! {
        Camera2d
    }
}
