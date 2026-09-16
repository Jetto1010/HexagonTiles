use bevy::prelude::*;
use crate::grid::Grid;

#[derive(Component)]
pub struct Puzzle{
    seed: u64,
    grid: Grid,
}