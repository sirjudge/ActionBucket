use crate::{
    SceneState, game::state::GameState, systems::SceneSwapMessage, ui::components::spawn_button,
};
use bevy::{
    app::AppExit,
    prelude::*,
    ui_widgets::{Activate, observe},
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

#[derive(Component)]
struct NewLobbyButton;

#[derive(Component)]
struct JoinLobbyButton;

#[derive(Component)]
struct SettingButton;

#[derive(Component)]
struct QuitToDesktopButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CurrentlySelectedOption::NewLobby)
            .add_systems(Update, Self::display_toggle_on_state_update);
    }
}

impl MainMenuPlugin {
    fn display_toggle_on_state_update(
        mut commands: Commands,
        interface_state: ResMut<State<SceneState>>,
        menu_query: Query<Entity, With<MainMenuUi>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        match menu_query.single() {
            Ok(menu) => {
                // if we get the query that's spawned and we are in the current
                // lobby menu state then no work required
                if interface_state.get() == &SceneState::MainMenu {
                    return;
                }

                // if we aren't in the lobby menu state despawn the menu
                commands.entity(menu).despawn();
            }
            Err(error) => {
                if interface_state.get().ne(&SceneState::MainMenu) {
                    return;
                }

                error!("{:?}", error);
                Self::spawn_main_menu(commands);
                game_state.set(GameState::Menu);
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
                    column_gap: px(5),
                    row_gap: px(5),
                    ..default()
                },
                MainMenuUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                        NewLobbyButton,
                        spawn_button("New Lobby"),
                        observe(
                            |_activate: On<Activate>,
                            mut message_writer: MessageWriter<SceneSwapMessage>| {
                                let message = SceneSwapMessage::new(SceneState::CreateLobby);
                                message_writer.write(message);
                            },
                        ),
                ));

                parent.spawn((
                        JoinLobbyButton,
                        spawn_button("Join Lobby"),
                        observe(
                            |_activate: On<Activate>,
                            mut message_writer: MessageWriter<SceneSwapMessage>| {
                                let message = SceneSwapMessage::new(SceneState::JoinLobby);
                                message_writer.write(message);
                                //TODO: need to do lobby connection stuff
                            },
                        ),
                ));

                parent.spawn((
                        SettingButton,
                        spawn_button("Settings"),
                        observe(
                            |_activate: On<Activate>,
                            mut message_writer: MessageWriter<SceneSwapMessage>| {
                                let message = SceneSwapMessage::new(SceneState::Settings);
                                message_writer.write(message);
                            },
                        ),
                ));

                parent.spawn((
                    spawn_button("Quit to Desktop"),
                    QuitToDesktopButton,
                    observe(
                        |_activate: On<Activate>, mut message_writer: MessageWriter<AppExit>| {
                            info!("Quit to Desktop button pushed");
                            message_writer.write(AppExit::Success);
                        },
                    ),
                ));
            });
    }
}
