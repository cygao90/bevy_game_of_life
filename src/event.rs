use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::prelude::*;
use bevy::log;

use crate::components::{Coordinate, CellState};
use crate::resources::{Board, BoardOptions, CellCollections};

#[derive(Debug, Copy, Clone, Event)]
pub struct CellTriggerEvent(pub Coordinate);

pub fn input_handling(
    windows: Query<&Window>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut cell_trigger_ewr: EventWriter<CellTriggerEvent>,
) {
    let window = windows.single();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::info!("Mouse button pressed: {:?} at {}", event.button, pos);
                let cell_coord = board.mouse_position(window, pos);
                if let Some(coord) = cell_coord {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("select on {}", coord);
                            cell_trigger_ewr.send(CellTriggerEvent(coord));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn trigger_event_handler(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut cell_collections: ResMut<CellCollections>,
    mut cell_trigger_evr: EventReader<CellTriggerEvent>,
    parent_query: Query<&Parent>,
    board_options: Res<BoardOptions>,
) {
    for trigger_event in cell_trigger_evr.iter() {
        let coord = trigger_event.0;
        let old_entity = cell_collections.get_selected_cell(&coord).cloned();
        if let Some(entity) = old_entity {
            let new_cell_state = match board.cell_map.map[coord.x][coord.y] {
                CellState::DEAD => CellState::ALIVE,
                CellState::ALIVE => CellState::DEAD,
            };
            if let Ok(parent) = parent_query.get(entity) {
                commands.entity(entity).despawn();
                commands.get_or_spawn(parent.get())
                    .with_children(|parent| {
                        let entity = parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: match new_cell_state {
                                    CellState::ALIVE => Color::WHITE,
                                    CellState::DEAD => Color::BLACK,
                                },
                                custom_size: Some(Vec2::splat(board_options.cell_size - board_options.cell_padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                coord.x as f32 * board_options.cell_size + (board_options.cell_size / 2.),
                                coord.y as f32 * board_options.cell_size + (board_options.cell_size / 2.),
                                1.
                            ),
                            ..default()
                        })
                            .insert(coord.clone())
                            .id();
                        cell_collections.update_collection(coord, entity);
                    });
            }
            board.cell_map.map[coord.x][coord.y] = new_cell_state;
            // commands.entity(entity).despawn();
        }
    }
}