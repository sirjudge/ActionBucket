use bevy::prelude::*;

use crate::scenes::{MenuSwapPlugin, lobby::LobbyPlugin, main_menu::MainMenuPlugin};

pub mod game;
pub mod infra;
pub mod scenes;
pub mod lobby;
pub mod tracking;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceState {
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

        //TODO: Log better later
        info!("created new game window");

        // Set the base interface state to main menu
        //TODO: Need to do validation here
        app.insert_state(InterfaceState::MainMenu);
        info!("set state to interface state");

        //TODO: Might be better to initialize these to false so we can just pass the
        //MainMenuPlugin without also having to initialize each one here.
        app.add_plugins((
            infra::audio::plugin,
            LobbyPlugin,
            MainMenuPlugin,
            MenuSwapPlugin,
        ));

        //TODO: This is something Bevy tutorial recommended but feels heavier handed than I need I think?
        // I'm keeping them in for now but need to come back at second glance
        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                // AppSystems::TickTimers,
                // AppSystems::RecordInput,
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
    //TODO: These came with the bevy tutorial so don't need for now
    // they look interesting and tickTimers and recordInput sound
    // like things I want to tinker with
    // TickTimers,
    // RecordInput,
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
