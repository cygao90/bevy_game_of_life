use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::prelude::*;
use bevy::log;

use crate::GameState;
use crate::components::{Coordinate, CellState};
use crate::resources::{Board, BoardOptions, CellCollections, LifeTimer};

#[derive(Debug, Copy, Clone, Event)]
pub struct CellTriggerEvent(pub Coordinate);

#[derive(Debug, Clone, Event)]
pub struct CellUpdateEvent {
    pub coord: Coordinate,
    pub state: CellState,
}

pub fn mouse_input_handling(
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

pub fn update_life(
    board: Res<Board>,
    mut cell_update_ewr: EventWriter<CellUpdateEvent>,
    mut life_timer: ResMut<LifeTimer>,
    time: Res<Time>,
) {
    if life_timer.0.tick(time.delta()).finished() {
        let events: Vec<_> = board.cell_map.iter().enumerate()
            .flat_map(|(y, states), | {
                states.iter().enumerate().map( move |(x, state)| {
                    Coordinate { x: x, y: y}
                })
            })
            .filter_map(|coord| {
                let alive_neighbors_count = board.count_neighbors(coord);

                let is_alive = board.is_alive(coord);
                let can_survive = is_alive && (alive_neighbors_count == 2 || alive_neighbors_count == 3);
                let can_live = !is_alive && alive_neighbors_count == 3;

                if is_alive && !can_survive {
                    Some(CellUpdateEvent {
                        coord: coord,
                        state: CellState::DEAD,
                    })
                } else if !is_alive && can_live {
                    Some(CellUpdateEvent {
                        coord: coord,
                        state: CellState::ALIVE,
                    })
                } else {
                    None
                }
            })
            .collect();
        
        for event in events {
            cell_update_ewr.send(event);
        }
    }
}

pub fn trigger_event_handler(
    board: Res<Board>,
    mut cell_trigger_evr: EventReader<CellTriggerEvent>,
    mut cell_update_ewr: EventWriter<CellUpdateEvent>,
) {
    for trigger_event in cell_trigger_evr.iter() {
        let coord = trigger_event.0;
            let new_cell_state = match board.cell_map.map[coord.x][coord.y] {
            CellState::DEAD => CellState::ALIVE,
            CellState::ALIVE => CellState::DEAD,
        };
        cell_update_ewr.send(CellUpdateEvent { coord: coord, state: new_cell_state });
    }
}

pub fn update_event_handler(
    mut commands: Commands,
    mut cell_update_evr: EventReader<CellUpdateEvent>,
    mut cell_collections: ResMut<CellCollections>,
    parent_query: Query<&Parent>,
    board_options: Res<BoardOptions>,
    mut board: ResMut<Board>,
) {
    for update_event in cell_update_evr.iter() {
        let (coord, new_cell_state) = (update_event.coord, &update_event.state);
        if let Some(entity) = cell_collections.get_selected_cell(&coord) {
            if let Ok(parent) = parent_query.get(*entity) {
                commands.entity(*entity).despawn();
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
                        board.cell_map.map[coord.x][coord.y] = new_cell_state.clone();
                    });
            }
        }
    }
}

pub fn key_board_input_handling(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::RUNNING => next_state.set(GameState::INITIAL),
            GameState::INITIAL => next_state.set(GameState::RUNNING),
        }
    }
}