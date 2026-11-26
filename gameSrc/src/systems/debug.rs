use crate::{GameState, SceneState, lobby::lobby::Lobby, systems::lobby_server::ServerState};
use bevy::{
    app::Plugin,
    diagnostic::LogDiagnosticsPlugin,
    ecs::system::Commands,
    prelude::Component,
    prelude::*,
    state::{app::AppExtStates, state::States},
    ui::Node,
};

#[derive(Component)]
pub struct ServerInfoUi;

#[derive(Component)]
pub struct LobbyInfoUi;

#[derive(Component)]
pub struct GameStateTextValue;

#[derive(Component)]
pub struct SceneStateTextValue;

#[derive(Component)]
pub struct FPSCountTextValue;

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
                (
                    Self::debug_ui_update,
                    Self::debug_lobby_update.run_if(resource_exists::<Lobby>),
                    Self::server_ui_update.run_if(in_state(ServerState::Started)),
                )
                    .run_if(in_state(DebugModeState::On)),
            ),
        );
    }
}

impl DebugPlugin {
    fn despawn_ui(mut commands: Commands) {
        commands.despawn_all::<With<DebugUi>>();
    }

    fn toggle_debug_ui(
        debug_mode_state: Res<State<DebugModeState>>,
        mut commands: Commands,
        debug_ui_query: Query<&DebugUi>,
    ) {
        match debug_mode_state.get() {
            DebugModeState::On => {
                if debug_ui_query.is_empty() {
                    commands.spawn((
                        Node {
                            justify_content: JustifyContent::Start,
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

                    commands.spawn((
                        Node {
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            top: percent(10),
                            right: percent(10),
                            position_type: PositionType::Absolute,
                            max_width: percent(25),
                            ..default()
                        },
                        DebugUi,
                        children![
                            (Text::new("Lobby Info"), TextFont::from_font_size(15.0)),
                            (LobbyInfoUi, Text::new("--"), TextFont::from_font_size(15.0)),
                        ],
                    ));

                    commands.spawn((
                        Node {
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            bottom: percent(10),
                            right: percent(10),
                            position_type: PositionType::Absolute,
                            max_width: percent(25),
                            ..default()
                        },
                        DebugUi,
                        children![
                            (Text::new("Server info"), TextFont::from_font_size(15.0)),
                            (
                                ServerInfoUi,
                                Text::new("--"),
                                TextFont::from_font_size(15.0)
                            ),
                        ],
                    ));
                }
            }
            DebugModeState::Off => {
                Self::despawn_ui(commands);
            }
        }
    }

    fn debug_lobby_update(
        lobby: Res<Lobby>,
        mut lobby_text_query: Single<&mut Text, With<LobbyInfoUi>>,
    ) {
        if lobby_text_query.is_empty() {
            warn!("could not find lobby text to update for debug ui");
            return;
        }

        let lobby_text_string = format!("{:?}", lobby);
        lobby_text_query.0 = lobby_text_string;
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
        if !game_state_text_query.is_empty() {
            game_state_text_query.0 = format!("{:?}", game_state.get());
        }

        if !scene_state_text_query.is_empty() {
            scene_state_text_query.0 = format!("{:?}", scene_state.get());
        }
    }

    fn server_ui_update(
        current_server_state: Res<State<ServerState>>,
        mut server_info_ui_query: Single<&mut Text, With<ServerInfoUi>>,
    ) {
        if server_info_ui_query.is_empty() {
            warn!("could not find server_info_ui_query");
        }

        server_info_ui_query.0 = format!("Server State:{:?}", current_server_state.get());
    }
}
