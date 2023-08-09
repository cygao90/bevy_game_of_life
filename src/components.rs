use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum CellState {
    DEAD,
    ALIVE,
}

#[derive(Debug, Clone, Component)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}