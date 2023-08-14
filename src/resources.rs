use std::ops::{Deref, DerefMut};
use bevy::{prelude::*, utils::HashMap, log};
use crate::{components::{CellState, Coordinate}};

const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];

#[derive(Debug, Clone, Resource)]
pub struct BoardOptions {
    pub map_size: (usize, usize),
    pub cell_size: f32,
    pub cell_padding: f32,
}

#[derive(Debug, Clone, Resource)]
pub struct CellMap {
    pub map: Vec<Vec<CellState>>,
}

#[derive(Debug, Clone, Resource)]
pub struct Bounds {
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Debug, Clone, Resource)]
pub struct CellCollections(pub HashMap<Coordinate, Entity>);

#[derive(Debug, Clone, Resource)]
pub struct Board {
    pub cell_map: CellMap,
    pub bounds: Bounds,
    pub cell_size: f32,
}

#[derive(Resource)]
pub struct LifeTimer(pub Timer);

impl Board {
    pub fn mouse_position(&self, window: &Window, mut position: Vec2) -> Option<Coordinate> {
        let window_size = Vec2::new(window.width(), window.height());
        position.y = window.height() - position.y;
        let position = position - window_size / 2.;

        if !self.bounds.in_bounds(position) {
            return None;
        }

        let coordinate = position - self.bounds.position;
        Some(Coordinate { x: (coordinate.x / self.cell_size) as usize, y: (coordinate.y / self.cell_size) as usize })
    }

    pub fn count_neighbors(&self, position: Coordinate) -> usize {
        SQUARE_COORDINATES.into_iter().filter(|direction| {
            let neighbor = position + *direction;
            self.coord_available(neighbor) && self.is_alive(neighbor)
        })
            .count()
    }

    pub fn is_alive(&self, coord: Coordinate) -> bool {
        self.cell_map[coord.x][coord.y] == CellState::ALIVE
    }

    fn coord_available(&self, coord: Coordinate) -> bool {
        return coord.x >= 0 
            && coord.y >= 0 
            && coord.x < self.cell_map[0].len()
            && coord.y < self.cell_map.len()
    }
}

impl CellCollections {
    pub fn get_selected_cell(&self, coord: &Coordinate) -> Option<&Entity> {
        self.0.get(coord)
    }

    pub fn update_collection(&mut self, coord: Coordinate, entity: Entity) {
        self.0.insert(coord, entity);
    }
}

impl Bounds {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (80, 80),
            cell_size: 10.,
            cell_padding: 0.2,
        }
    }
}

impl CellMap {
    pub fn empty(width: usize, height: usize) -> Self {
        let map = (0..height).into_iter()
            .map(|_| (0..width).into_iter().map(|_| CellState::DEAD).collect()).collect();
        Self {
            map: map,
        }
    }
}

impl Deref for CellMap {
    type Target = Vec<Vec<CellState>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for CellMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}