use crate::{InterfaceState, interface::menu_switcher::MenuSwapMessage};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button, observe},
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentlySelectedOption {
    #[default]
    NewLobby,
    JoinLobby,
    Settings,
    Exit,
}

#[derive(Component)]
pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenuUi;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CurrentlySelectedOption::NewLobby)
            .add_systems(Update, Self::display_toggle_on_state_update);
    }
}

impl MainMenuPlugin {
    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<InterfaceState>>,
        menu_query: Query<Entity, With<MainMenuUi>>,
    ) {
        match menu_query.single() {
            Ok(menu) => {
                // if we get the query that's spawned and we are in the current
                // lobby menu state then no work required
                if interface_state.get() == &InterfaceState::MainMenu {
                    return;
                }

                // if we aren't in the lobby menu state despawn the menu
                commands.entity(menu).despawn();
            }
            Err(error) => {
                if interface_state.get().ne(&InterfaceState::MainMenu) {
                    return;
                }

                error!("{:?}", error);
                Self::spawn_main_menu(commands);
            }
        }
    }

    fn spawn_main_menu(mut commands: Commands) {
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
                MainMenuUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                Button,
                Text::new("New Lobby"),
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("New Lobby button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::CreateLobby);
                        message_writer.write(message);
                    },
                ),
                Node { ..default() },
            ));

                parent.spawn((
                Button,
                Text::new("Join Lobby"),
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("Join Lobby button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::JoinLobby);
                        message_writer.write(message);
                    },
                ),
                Node { ..default() },
            ));

                parent.spawn((
                Button,
                Text::new("Settings"),
                observe(
                    |_activate: On<Activate>,
                     mut message_writer: MessageWriter<MenuSwapMessage>| {
                        info!("Settinsg button pushed");
                        let message = MenuSwapMessage::new(InterfaceState::Settings);
                        message_writer.write(message);
                    },
                ),
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
    }
}
