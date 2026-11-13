use bevy::prelude::*;

use crate::{interface::{lobby_menu::LobbyPlugin, main_menu::MainMenuPlugin}, lobby::game_lobby::Lobby};

pub mod game;
pub mod infra;
pub mod interface;
pub mod lobby;
pub mod tracking;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceState {
    #[default]
    MainMenu,
    JoinLobby,
    CreateLobby,
    InLobby,
    InGame,
    Loading,
    Startup,
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
        )
        .insert_state(InterfaceState::MainMenu);

        // Add other plugins.
        app.add_plugins((
            infra::audio::plugin,
            LobbyPlugin,
            MainMenuPlugin
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera.spawn());
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera() -> impl Scene {
    bsn! {
        Camera2d
    }
}
