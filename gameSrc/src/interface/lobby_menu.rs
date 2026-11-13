use crate::{InterfaceState, interface::main_menu::MainMenuPlugin, lobby::game_lobby::Lobby};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

#[derive(Component)]
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    // using this as base
    // https://bevy.org/examples/ui-user-interface/standard-widgets/
    fn build(&self, app: &mut App) {
        app.insert_state::<CurrentlySelectedFighter>(CurrentlySelectedFighter::Random)
            .insert_resource::<Lobby>(Lobby::default())
            .add_systems(Startup, spawn_lobby.run_if(in_state(InterfaceState::InLobby)))
            .add_systems(Update, despawn_menu);
    }
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedFighter {
    #[default]
    Random = 0,
    BucketOne = 1,
    BucketTwo = 2,
    BucketThree = 3,
    BucketFour = 4,
}

fn spawn_lobby(
    mut commands: Commands,
    state: ResMut<State<InterfaceState>>,
    mut next_state: ResMut<NextState<InterfaceState>>,
) {
    if state.eq(&InterfaceState::InLobby) {
        debug!("already in lobby menu, no spawning needed");
    }

    commands
        .spawn((Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Node { ..default() },
                Button,
                Name::new("ActionBucket1"),
                Text::new("Action Bucket"),
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

fn despawn_menu(
    mut commands: Commands,
    state: ResMut<State<InterfaceState>>,
    menu_query: Query<Entity, With<LobbyPlugin>>,
) {
    // if the current interface is lobby + we can get the root menu entity despawn
    if state.eq(&InterfaceState::InLobby) && let Ok(_) = menu_query.single() {
        commands.despawn_all::<With<LobbyPlugin>>();
    }
}

