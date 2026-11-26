use crate::{
    SceneState, lobby::{lobby::Lobby, player::PlayerConnectionType}, systems::{SceneSwapMessage, lobby_server::{ConnectionAction, PlayerConnectionMessage}}, ui::components::spawn_button,
};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Component)]
struct ActionBucket1Button;
#[derive(Component)]
struct ActionBucket2Button;
#[derive(Component)]
struct ActionBucket3Button;
#[derive(Component)]
struct ActionBucket4Button;
#[derive(Component)]
struct StartMatchButton;
#[derive(Component)]
struct MatchSettingsButton;
#[derive(Component)]
struct QuitToMainMenuButton;

#[derive(Component)]
pub struct LobbyPlugin;

#[derive(Component)]
pub struct LobbyUi;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::display_toggle_on_state_update);
    }
}

impl LobbyPlugin {
    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<SceneState>>,
        lobby_query: Query<Entity, With<LobbyUi>>,
    ) {
        match lobby_query.single() {
            Ok(lobby_ui) => {
                // lobby UI is already spawned and we're still in the
                // same lobby state, no work required
                if interface_state.get() == &SceneState::InLobby {
                    return;
                }

                if interface_state.get().ne(&SceneState::InLobby) {
                    commands.entity(lobby_ui).despawn();
                    debug!("Successfully despawned lobby menu");
                }
            }
            Err(error) => {
                // if we get an error extracting the menu and we're not supposed
                // to be in the lobby menu then no work required
                if interface_state.get().ne(&SceneState::InLobby) {
                    return;
                }

                // if we couldn't find the interface and we're in the lobby
                // we should log error and spawn
                warn!(
                    "interface state is in lobby but could not find lobby menu:{:?}",
                    error
                );
                Self::spawn_lobby(commands);
            }
        }
    }

    fn spawn_lobby(mut commands: Commands) {
        commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10),
                    column_gap: px(10),
                    ..default()
                },
                LobbyUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                    ActionBucket1Button,
                    spawn_button("Action Bucket 1"),
                    Name::new("ActionBucket1"),
                    observe(|_activate: On<Activate> | {
                        info!("action bucket 1 selected");
                    }),
                ));
                parent.spawn((
                    ActionBucket2Button,
                    spawn_button("Action Bucket 2"),
                    Name::new("ActionBucket2"),
                    observe(|_activate: On<Activate> | {
                        info!("action bucket 2 selected");
                    }),
                ));
                parent.spawn((
                    ActionBucket3Button,
                    Name::new("ActionBucket3"),
                    spawn_button("Action Bucket 3"),
                    observe(|_activate: On<Activate> | {
                        info!("action bucket 3 selected");
                    }),
                ));
                parent.spawn((
                    ActionBucket4Button,
                    Name::new("ActionBucket4"),
                    spawn_button("Action Bucket 3"),
                    observe(|_activate: On<Activate> | {
                        info!("action bucket 4 selected");
                    }),
                ));

                parent
                    .spawn((Node {
                        width: percent(75),
                        height: percent(20),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        row_gap: px(10),
                        column_gap: px(10),
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn((
                            StartMatchButton,
                            spawn_button("Start Game"),
                            Name::new("StartGameButton"),
                            observe(|_activate: On<Activate>, mut message_writer: MessageWriter<SceneSwapMessage> | {
                                info!("Start game button pushed");
                                message_writer
                                    .write(SceneSwapMessage { next_state: SceneState::InGame});
                            }),
                        ));

                        parent.spawn((
                            MatchSettingsButton,
                            spawn_button("Change Match Settings"),
                            Name::new("MatchSettingsButton"),
                            observe(|_activate: On<Activate>, mut message_writer: MessageWriter<SceneSwapMessage> | {
                                info!("settings button pushed");
                                message_writer
                                    .write(SceneSwapMessage { next_state: SceneState::Settings});
                            }),
                        ));

                        parent.spawn((
                            QuitToMainMenuButton,
                            spawn_button("Quit to Main Menu"),
                            Name::new("QuitToMenuButton"),
                            observe(|
                                _activate: On<Activate>,
                                mut scene_swap_message_writer: MessageWriter<SceneSwapMessage>,
                                mut player_connection_message_writer: MessageWriter<PlayerConnectionMessage>,
                                lobby: Res<Lobby>
                            | {
                                info!("quit to menu button pushed");

                                for index in 0..lobby.local_ports.len() {
                                    let player_port_to_disconnect = &lobby.local_ports[index];
                                    match  lobby.port_player_map.get(player_port_to_disconnect) {
                                       Some(player_option) =>{
                                           if let Some(player) = player_option {
                                            let disconnect_message = PlayerConnectionMessage {
                                                player: player.clone(),
                                                connection_message_type: ConnectionAction::Disconnect,
                                                connection_type: PlayerConnectionType::Local
                                             };
                                            player_connection_message_writer.write(disconnect_message);
                                           }
                                       }
                                       None =>{}
                                    }
                                }

                                scene_swap_message_writer.write(SceneSwapMessage {next_state:SceneState:: MainMenu});
                                //TODO: need to see if locally connected player port is also
                                //hosting the lobby and quit the server accordingly 
                            }),
                        ));
                    });
            });
    }
}
