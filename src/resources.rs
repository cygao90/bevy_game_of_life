use std::ops::{Deref, DerefMut};
use bevy::prelude::*;
use crate::components::{CellState, Coordinate};


#[derive(Debug, Clone, Resource)]
pub struct BoardOptions {
    // pub map_size: (usize, usize),
    pub cell_size: f32,
    pub cell_padding: f32,
}

#[derive(Debug, Clone, Resource)]
pub struct CellMap {
    pub map: Vec<Vec<CellState>>,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            // map_size: (100, 100),
            cell_size: 10.,
            cell_padding: 0.5,
        }
    }
}

impl CellMap {
    pub fn empty(width: usize, height: usize) -> Self {
        let map = (0..height).into_iter()
            .map(|_| (0..width).into_iter().map(|_| CellState::DEAD).collect()).collect();
        Self {
            map,
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