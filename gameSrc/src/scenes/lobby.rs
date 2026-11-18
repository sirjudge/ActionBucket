use crate::{InterfaceState, lobby::game_lobby::Lobby};
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
struct LobbyUi;

// https://bevy.org/examples/ui-user-interface/standard-widgets/
// using this as base
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state::<CurrentFighterOption>(CurrentFighterOption::Random)
            .insert_resource::<Lobby>(Lobby::default());

        app.add_systems(
            Update,
            (Self::display_toggle_on_state_update, Self::lobby_debug),
        );
    }
}

impl LobbyPlugin {
    fn lobby_debug(selected_fighter_option: ResMut<State<CurrentFighterOption>>) {
        debug!("currently selected_fighter_option: {:?}", selected_fighter_option);
    }

    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<InterfaceState>>,
        next_state: ResMut<NextState<InterfaceState>>,
        lobby_query: Query<Entity, With<LobbyUi>>,
    ) {
        match lobby_query.single() {
            Ok(lobby_ui) => {
                // lobby UI is already spawned and we're still in the
                // same lobby state, no work required
                if interface_state.get() == &InterfaceState::InLobby {
                    return;
                }

                if interface_state.get().ne(&InterfaceState::InLobby) {
                    commands.entity(lobby_ui).despawn();
                    debug!("Successfully despawned lobby menu");
                }
            }
            Err(error) => {
                // if we get an erro extracting the menu and we're not supposed
                // to be in the lobby menu then no work required
                if interface_state.get().ne(&InterfaceState::InLobby) {
                    return;
                }

                // if we couldn't find the interface and we're in the lobby
                // we should log error and spawn
                warn!(
                    "interface state is in lobby but could not find lobby menu:{:?}",
                    error
                );
                // Self::spawn_lobby(commands, next_state, next_fighter_option_state);
                Self::spawn_lobby(commands, next_state);
            }
        }
    }

    fn spawn_lobby(
        mut commands: Commands,
        mut next_state: ResMut<NextState<InterfaceState>>,
        // mut next_fighter_option_state: ResMut<NextState<CurrentFighterOption>>,
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
                            observe(|_activate: On<Activate>, mut next_state: ResMut<NextState<InterfaceState>> | {
                                info!("Start game button pushed");
                                next_state.set(InterfaceState::InGame);
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("SettingsButton"),
                            Text::new("Change Match Settings"),
                            observe(|_activate: On<Activate>, mut next_state: ResMut<NextState<InterfaceState>> | {
                                info!("settings button pushed");
                                next_state.set(InterfaceState::Settings);
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("QuitToMenuButton"),
                            Text::new("Quit To Main Menu"),
                            observe(|_activate: On<Activate>, mut next_state: ResMut<NextState<InterfaceState>> | {
                                info!("quit to menu button pushed");
                                next_state.set(InterfaceState::MainMenu);
                            }),
                        ));
                    });
            });
        next_state.set(InterfaceState::InLobby);
        info!("Successfully ran spawn in lobby ui to completion");
    }
}
