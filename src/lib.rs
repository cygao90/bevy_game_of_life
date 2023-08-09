use bevy::prelude::*;
use resources::{BoardOptions, CellMap};
use bevy::log;

use crate::components::Coordinate;

mod components;
mod resources;

pub struct BoardPlugin;

pub struct Bounds {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardOptions {
            ..Default::default()
        })
            .add_systems(Startup, Self::setup);
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
        let (board_width, board_height) = (window.width(), window.height()); // board_options.map_size;
        let cell_size = board_options.cell_size;
        let cell_map = CellMap::empty(
            (board_width / cell_size) as usize,
            (board_height / cell_size) as usize
        );

        log::info!("window size: {} * {}", window.width(), window.height());
        log::info!("cell size: {}", cell_size);
        log::info!("cell number: {}", cell_map.len() * cell_map[0].len());

        commands.spawn(SpriteBundle {
            sprite: Sprite { 
                color: Color::GRAY,
                custom_size: Some(
                    Vec2::new(
                        (board_width * cell_size) as f32,
                        (board_height * cell_size) as f32
                    )
                ),
                ..default() 
            },
            visibility: Visibility::Visible,
            ..default()
        })
            .insert(Name::new("Board"))
            .insert(Transform::from_xyz(
                    -(board_width as f32 / 2.),
                    -(board_height as f32 / 2.),
                    0.
                )
            )
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                for (y, line) in cell_map.iter().enumerate() {
                    for (x, _cell) in line.iter().enumerate() {
                        // log::info!("cell: ({x}, {y})");
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLACK,
                                custom_size: Some(Vec2::splat(cell_size - board_options.cell_padding)),
                                ..Default::default()
                            },
                            visibility: Visibility::Visible,
                            transform: Transform::from_xyz(
                                x as f32 * cell_size + (cell_size / 2.),
                                y as f32 * cell_size + (cell_size / 2.),
                                1.
                            ),
                            ..Default::default()
                        })
                            .insert(Coordinate {
                                x: x,
                                y: y,
                            });
                    }
                }
            });
    }
}