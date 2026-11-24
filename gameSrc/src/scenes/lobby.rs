use crate::{
    SceneState,
    lobby::{game_lobby::Lobby, player::Player},
    scenes::SceneSwapMessage,
};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentFighterOption {
    #[default]
    Random = 0,
    BucketOne = 1,
    BucketTwo = 2,
    BucketThree = 3,
    BucketFour = 4,
}

#[derive(Component)]
pub struct LobbyPlugin;

#[derive(Component)]
pub struct LobbyUi;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state::<CurrentFighterOption>(CurrentFighterOption::Random)
            .insert_resource::<Lobby>(Lobby::default())
            .add_message::<PlayerConnectionMessage>();

        app.add_systems(PostStartup, Self::init_lobby_post_startup);
        app.add_systems(Update, Self::display_toggle_on_state_update);
        app.add_systems(
            Update,
            Self::player_event_reader.run_if(in_state(SceneState::InLobby)),
        );
    }
}

pub enum ConnectionMessageType {
    Connect,
    Disconnect,
}

#[derive(Message)]
pub struct PlayerConnectionMessage {
    player: Player,
    connection_message_type: ConnectionMessageType,
}

impl LobbyPlugin {
    fn player_event_reader(
        mut connection_message_reader: MessageReader<PlayerConnectionMessage>,
        mut lobby: ResMut<Lobby>,
    ) {
        for message in connection_message_reader.read() {
            match message.connection_message_type {
                ConnectionMessageType::Connect => {
                    info!(
                        "Processing player connection for player:{:?}",
                        message.player
                    );
                    lobby.add_player_to_list(&message.player);
                }
                ConnectionMessageType::Disconnect => {
                    info!(
                        "Processing player disconnection for player:{:?}",
                        message.player
                    );
                    match lobby.disconnect_player_by_player(&message.player) {
                        Ok(_) => {
                            info!("Successfully disconnected player from lobby");
                        }
                        Err(error) => {
                            error!("failed disconnecting player from lobby:{:?}", error);
                        }
                    }
                }
            }
        }
    }

    fn init_lobby_post_startup(mut lobby_resource: ResMut<Lobby>) {
        lobby_resource.initialize_player_list();
    }

    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<SceneState>>,
        lobby_query: Query<Entity, With<LobbyUi>>,
        menu_swap_message_writer: MessageWriter<SceneSwapMessage>,
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
                // if we get an erro extracting the menu and we're not supposed
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
                Self::spawn_lobby(commands, menu_swap_message_writer);
            }
        }
    }

    fn spawn_lobby(
        mut commands: Commands,
        mut menu_swap_message_writer: MessageWriter<SceneSwapMessage>,
    ) {
        commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                LobbyUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket1"),
                    Text::new("Action Bucket 1"),
                    observe(|_activate: On<Activate>,mut next_fighter_option_state: ResMut<'_, NextState<CurrentFighterOption>> | {
                        info!("action bucket 1 selected");
                        next_fighter_option_state.set(CurrentFighterOption::BucketOne);
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket2"),
                    Text::new("Action Bucket 2"),
                    observe(|_activate: On<Activate>,mut next_fighter_option_state: ResMut<'_, NextState<CurrentFighterOption>> | {
                        info!("action bucket 2 selected");
                        next_fighter_option_state.set(CurrentFighterOption::BucketTwo);
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket3"),
                    Text::new("Action Bucket 3"),
                    observe(|_activate: On<Activate>,mut next_fighter_option_state: ResMut<'_, NextState<CurrentFighterOption>> | {
                        info!("action bucket 3 selected");
                        next_fighter_option_state.set(CurrentFighterOption::BucketThree);
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket4"),
                    Text::new("Action Bucket 4"),
                    observe(|_activate: On<Activate>,mut next_fighter_option_state: ResMut<'_, NextState<CurrentFighterOption>> | {
                        info!("action bucket 4 selected");
                        next_fighter_option_state.set(CurrentFighterOption::BucketFour);
                    }),
                ));

                parent
                    .spawn((Node {
                        width: percent(75),
                        height: percent(20),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("StartGameButton"),
                            Text::new("Start Game"),
                            observe(|_activate: On<Activate>, mut message_writer: MessageWriter<SceneSwapMessage> | {
                                info!("Start game button pushed");
                                message_writer
                                    .write(SceneSwapMessage { next_state: SceneState::InGame});
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("SettingsButton"),
                            Text::new("Change Match Settings"),
                            observe(|_activate: On<Activate>, mut message_writer: MessageWriter<SceneSwapMessage> | {
                                info!("settings button pushed");
                                message_writer
                                    .write(SceneSwapMessage { next_state: SceneState::Settings});
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("QuitToMenuButton"),
                            Text::new("Quit To Main Menu"),
                            observe(|_activate: On<Activate>, mut message_writer: MessageWriter<SceneSwapMessage> | {
                                info!("quit to menu button pushed");
                                message_writer.write(SceneSwapMessage {next_state:SceneState:: MainMenu});
                            }),
                        ));
                    });
            });

        menu_swap_message_writer.write(SceneSwapMessage {
            next_state: SceneState::InLobby,
        });
        info!("Successfully ran spawn in lobby ui to completion");
    }
}
