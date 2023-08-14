use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    INITIAL,
    RUNNING,
}