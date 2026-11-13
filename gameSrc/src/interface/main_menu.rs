use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

use crate::InterfaceState;

#[derive(Component)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CurrentlySelectedOption::NewLobby)
            .add_systems(Startup, spawn_main_menu.run_if(in_state(InterfaceState::MainMenu)))
            .add_systems(Update,despawn_menu);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedOption {
    #[default]
    NewLobby,
    JoinLobby,
    Settings,
    Exit,
}

fn spawn_main_menu(
    mut commands: Commands,
    state: ResMut<State<InterfaceState>>,
    mut next_state: ResMut<NextState<InterfaceState>>,
) {

    // if states are the same no set up required?
    if state.eq(&InterfaceState::MainMenu){
        debug!("returned early from main menu set up because the state was already main menu")
    }

    commands.spawn(Camera2d);
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
                Button,
                Text::new("New Lobby"),
                observe(|_activate: On<Activate>| {
                    info!("New Lobby button pushed");
                    //TODO: need to despawn everything here and swap
                    //to lobby instead
                    next_state.set(InterfaceState::InLobby);
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Join Lobby"),
                observe(|_activate: On<Activate>| {
                    info!("Join Lobby button pushed");
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Settings"),
                observe(|_activate: On<Activate>| {
                    info!("Settinsg button pushed");
                }),
                Node { ..default() },
            ));

            parent.spawn((
                Button,
                Text::new("Quit to Desktop"),
                observe(|_activate: On<Activate>| {
                    info!("Quit to Desktop button pushed");
                }),
                Node { ..default() },
            ));
        });
    next_state.set(InterfaceState::MainMenu);
}


fn despawn_menu(
    mut commands: Commands,
    state: ResMut<State<InterfaceState>>,
    menu_query: Query<Entity, With<MainMenuPlugin>>,
) {
    // if the current interface is lobby + we can get the root menu entity despawn
    if state.eq(&InterfaceState::InLobby) && let Ok(_) = menu_query.single() {
        commands.despawn_all::<With<MainMenuPlugin>>();
    }
}
