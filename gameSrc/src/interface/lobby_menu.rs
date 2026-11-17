use crate::{InterfaceState, infra::log_source::LogSource::Interface, lobby::game_lobby::Lobby};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedFighter {
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

// https://bevy.org/examples/ui-user-interface/standard-widgets/
// using this as base
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state::<CurrentlySelectedFighter>(CurrentlySelectedFighter::Random)
            .insert_resource::<Lobby>(Lobby::default());

        app.add_systems(Update, Self::display_toggle_on_state_update);
    }
}

impl LobbyPlugin {
    // fn despawn_lobby(mut commands: Commands, ) {
    //     match lobby_query.single(){
    //         Ok(lobby_ui) =>{
    //
    //         },
    //         Err(error) =>{
    //
    //         }
    //     }
    // }
    //
    fn spawn_lobby(
        mut commands: Commands,
        interface_state: ResMut<State<InterfaceState>>,
        mut next_state: ResMut<NextState<InterfaceState>>,
    ) {
        if interface_state.eq(&InterfaceState::InLobby) {
            debug!("already in lobby menu, no spawning needed");
            return;
        }

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
                    observe(|_activate: On<Activate>| {
                        info!("action bucket 1 selected");
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket2"),
                    Text::new("Action Bucket 2"),
                    observe(|_activate: On<Activate>| {
                        info!("action bucket 2 selected");
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket3"),
                    Text::new("Action Bucket 3"),
                    observe(|_activate: On<Activate>| {
                        info!("action bucket 3 selected");
                    }),
                ));
                parent.spawn((
                    Node { ..default() },
                    Button,
                    Name::new("ActionBucket4"),
                    Text::new("Action Bucket 4"),
                    observe(|_activate: On<Activate>| {
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
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("StartGameButton"),
                            Text::new("Start Game"),
                            observe(|_activate: On<Activate>| {
                                info!("Start game button pushed");
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("SettingsButton"),
                            Text::new("Change Match Settings"),
                            observe(|_activate: On<Activate>| {
                                info!("settings button pushed");
                            }),
                        ));

                        parent.spawn((
                            Node { ..default() },
                            Button,
                            Name::new("QuitToMenuButton"),
                            Text::new("Quit To Main Menu"),
                            observe(|_activate: On<Activate>| {
                                info!("quit to menu button pushed");
                            }),
                        ));
                    });
            });
        next_state.set(InterfaceState::InLobby);
    }

    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<InterfaceState>>,
        next_state: ResMut<NextState<InterfaceState>>,
        lobby_query: Query<Entity, With<LobbyUi>>,
    ) {

        match lobby_query.single() {
            Ok(menu) => {
                // if we get the query that's spawned and we are in the current
                // lobby menu state then no work required
                if interface_state.get() == &InterfaceState::InLobby {
                    return;
                }

                // if we aren't in the lobby menu state despawn the menu
                commands.entity(menu).despawn();
            }
            Err(error) => {
                error!("{:?}",error);
                // if we get an erro extracting the menu and we're not supposed
                // to be in the lobby menu then no work required 
                if interface_state.get() == &InterfaceState::InLobby{
                    // need to spawn
                    Self::spawn_lobby(commands, interface_state, next_state);
                }
                else if interface_state.get().ne(&InterfaceState::InLobby) {
                    return;
                }
            }
        }
    }
}
