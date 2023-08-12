use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::prelude::*;
use bevy::log;

use crate::resources::Board;

pub fn input_handling(
    windows: Query<&Window>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.single();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let cell_coord = board.mouse_position(window, pos);
                if let Some(coord) = cell_coord {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("select on {}", coord);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}