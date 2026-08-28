use bevy::platform::collections::HashMap;
use bevy::prelude::*;


#[derive(Resource)]
pub struct Grid{
    pub cells: HashMap<HexCoordinates, CellState>,
}
impl Grid {
    pub fn new(radius: i8) -> Self {
        let mut cells = HashMap::new();

        for q in -radius..=radius {
            let r_min = (-radius).max(-q - radius);
            let r_max = radius.min(-q + radius);
            for r in r_min..=r_max {
                let s = -q - r;
                cells.insert(HexCoordinates{q, r, s}, CellState::Empty);
            }
        }
        
        Grid { cells }
    }
}


#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct HexCoordinates {
    pub q: i8,
    pub s: i8,
    pub r: i8,
}

#[derive(Component, Clone, PartialEq, Debug, Copy)]
pub enum CellState{
    Empty,
    Filled(Color),
}

pub fn draw_grid(
    mut commands: Commands,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
}