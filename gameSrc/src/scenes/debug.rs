use bevy::prelude::*;
use bevy::{
    app::Plugin,
    diagnostic::LogDiagnosticsPlugin,
    ecs::system::Commands,
    prelude::Component,
    state::{app::AppExtStates, state::States},
    ui::Node,
};

use crate::game::state::{GameState, SceneState};

#[derive(States, Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum DebugModeState {
    #[default]
    On,
    Off,
}

#[derive(Component)]
pub struct DebugUi;

#[derive(Component)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_state(DebugModeState::On)
            .add_plugins(LogDiagnosticsPlugin::default());

        app.add_systems(
            Update,
            (
                Self::toggle_debug_ui.run_if(state_changed::<DebugModeState>),
                Self::debug_ui_update.run_if(in_state(DebugModeState::On)),
            ),
        );
    }
}

impl DebugPlugin {
    fn spawn_ui(mut commands: Commands) {
        commands
            .spawn((
                Node {
                    // align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    bottom: percent(10),
                    left: percent(10),
                    position_type: PositionType::Absolute,

                    ..default()
                },
                DebugUi,
                children![
                    (Text::new("Game State")),
                    (GameStateTextValue, Text::new("--")),
                    (Text::new("Scene State:")),
                    (SceneStateTextValue, Text::new("--")),
                ],
            ));
            // .with_children(|parent| {
            //     parent.spawn((
            //     ));
            //
            //     parent.spawn((
            //     ));
            // });
    }

    fn despawn_ui(mut commands: Commands) {
        commands.despawn_all::<With<DebugUi>>();
    }

    fn toggle_debug_ui(
        debug_mode_state: Res<State<DebugModeState>>,
        commands: Commands,
        debug_ui_query: Query<&DebugUi>,
    ) {
        match debug_mode_state.get() {
            DebugModeState::On => {
                if debug_ui_query.is_empty() {
                    Self::spawn_ui(commands);
                }
            }
            DebugModeState::Off => {
                Self::despawn_ui(commands);
            }
        }
    }

    fn debug_ui_update(
        game_state: Res<State<GameState>>,
        scene_state: Res<State<SceneState>>,
        mut game_state_text_query: Single<
            &mut Text,
            (With<GameStateTextValue>, Without<SceneStateTextValue>),
        >,
        mut scene_state_text_query: Single<
            &mut Text,
            (With<SceneStateTextValue>, Without<GameStateTextValue>),
        >,
    ) {

        if !game_state_text_query.is_empty(){
            let game_state_string = format!("{:?}", game_state.get());
            game_state_text_query.0 = game_state_string;
        }
        
        if !scene_state_text_query.is_empty(){
            let scene_state_string = format!("{:?}", scene_state.get());
            scene_state_text_query.0 = scene_state_string;
        }
        
        // match game_state_text_query  {
        //     // Ok(mut game_state_text_component) => {
        //     Ok(_) => {
        //         // let game_state_string = format!("{:?}", game_state.get());
        //         // game_state_text_component.0 = game_state_string;
        //     }
        //     Err(error) => {
        //         warn!(
        //             "debug menu enabled but could not find game state text component:{:?}",
        //             error
        //         );
        //     }
        // }
        // match scene_state_text_query.single_mut() {
        //     Ok(mut scene_state_text_component) => {
        //         let scene_state_string = format!("{:?}", scene_state.get());
        //         scene_state_text_component.0 = scene_state_string;
        //     }
        //     Err(error) => {
        //         warn!(
        //             "debug menu enabled but could not find game state text component:{:?}",
        //             error
        //         );
        //     }
        // }
    }
}

#[derive(Component)]
pub struct GameStateTextValue;

#[derive(Component)]
pub struct SceneStateTextValue;

#[derive(Component)]
pub struct FPSCountTextValue;
