use bevy::prelude::*;

use crate::InterfaceState;

#[derive(Message)]
pub struct MenuSwapMessage {
    next_state: InterfaceState,
}

impl MenuSwapMessage {
    pub fn new(next_state: InterfaceState) -> Self {
        MenuSwapMessage { next_state }
    }
}

#[derive(Component)]
pub struct MenuSwapPlugin;

impl Plugin for MenuSwapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MenuSwapMessage>()
            .add_systems(Update, swap_message_reader);
    }
}

fn swap_message_reader(
    mut message_reader: MessageReader<MenuSwapMessage>,
    state: ResMut<State<InterfaceState>>,
    mut next_state: ResMut<NextState<InterfaceState>>,
) {
    for message in message_reader.read() {
        // if we have the same states don't swap
        let log_message = format!("Recieved swap interface message: {:?}",message.next_state);
        info!(log_message);
        if &message.next_state == state.get() {
            return;
        }
        next_state.set(message.next_state.clone());
    }
}
