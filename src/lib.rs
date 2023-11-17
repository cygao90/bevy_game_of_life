use bevy::{prelude::*, utils::HashMap};
use resources::*;
use bevy::log;

use crate::components::Coordinate;
use crate::event::*;
use crate::state::GameState;

mod components;
mod resources;
mod event;
mod state;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardOptions {
            ..Default::default()
        })
            .add_state::<GameState>()
            .insert_resource(LifeTimer(Timer::from_seconds(
                0.2,
                TimerMode::Repeating,
            )))
            .add_systems(Startup, Self::setup)
            .add_systems(
                Update,
                (
                    mouse_input_handling,
                    trigger_event_handler
                ).run_if(in_state(GameState::INITIAL))
            )
            .add_systems(
                Update,
                (
                update_life,
                ).run_if(in_state(GameState::RUNNING))
            )
            .add_systems(Update, (key_board_input_handling, update_event_handler))
            .add_event::<CellTriggerEvent>()
            .add_event::<CellUpdateEvent>();
        log::info!("Loaded Board Plugin");
    }
}

// systems
impl BoardPlugin {
    pub fn setup(
        mut commands: Commands,
        board_options: Res<BoardOptions>,
        windows: Query<&Window>
    ) {
        let window = windows.single();
        let (board_width, board_height) = board_options.map_size;
        let cell_size = board_options.cell_size;
        let board_size = Vec2::new(
            board_width as f32 * cell_size,
            board_height as f32 * cell_size,
        );
        let cell_map = CellMap::empty(
            board_width,
            board_height,
        );
        let board_position = Vec3::new(
            -(board_size.x / 2.),
            -(board_size.y / 2.),
            0.
        );

        let mut cell_collections = HashMap::with_capacity(board_width * board_height);

        log::info!("window size: {} * {}", window.width(), window.height());
        log::info!("cell size: {}", cell_size);
        log::info!("cell number: {}", cell_map.len() * cell_map[0].len());

        commands.spawn(SpatialBundle {
            visibility: Visibility::Visible,
            transform: Transform::from_translation(board_position),
            ..Default::default()
        })
            .insert(Name::new("board"))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite { 
                        color: Color::Rgba { red: 255., green: 254., blue: 255., alpha: 0. },
                        custom_size: Some(
                            Vec2::new(
                                board_width as f32 * cell_size,
                                board_height as f32 * cell_size,
                            )
                        ),
                        ..default() 
                    },
                    transform: Transform::from_translation(-board_position),
                    global_transform: GlobalTransform::default(),
                    visibility: Visibility::Visible,
                    ..default()
                })
                    .insert(Name::new("background"));

                for (y, line) in cell_map.iter().enumerate() {
                    for (x, _cell) in line.iter().enumerate() {
                        // log::info!("cell: ({x}, {y})");
                        let coord = Coordinate {x: x, y: y};
                        let entity = parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLACK,
                                custom_size: Some(Vec2::splat(cell_size - board_options.cell_padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                x as f32 * cell_size + (cell_size / 2.),
                                y as f32 * cell_size + (cell_size / 2.),
                                1.
                            ),
                            ..Default::default()
                        })
                            .insert(coord.clone())
                            .id();
                        cell_collections.insert(coord, entity);
                    }
                }
            });

        commands.insert_resource(Board {
            cell_map: cell_map,
            bounds: Bounds { position: board_position.truncate(), size: board_size },
            cell_size: cell_size,
        });
        commands.insert_resource(CellCollections(cell_collections));
    }
}