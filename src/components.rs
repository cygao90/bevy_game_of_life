use bevy::prelude::*;

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum CellState {
    DEAD,
    ALIVE,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinate {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as isize) + x as isize) as usize;
        let y = ((self.y as isize) + y as isize) as usize;
        Self { x, y }
    }
}


impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}