use std::f32::consts::PI;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::RngExt;
use crate::color_list::ColorList;
use crate::color_selection::{ColorPlugin, CurrentColor};
use crate::numbers::{NumberPlugin, NumberIndicator};
use strum::{EnumCount};
use crate::shapes::{ShapesPlugin, ShapeList, Shape};
use crate::hexagon::{HexCoordinates, HEXAGON_SIZE, hexagon_mesh, flat_hex_to_pixel};

use super::{GameState};

pub const GRID_SIZE: f32 = 250.0; // Pointy-top -> size from center to top
pub const HEXAGONS_FROM_CENTER: f32 = 3.0;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app
            .add_plugins((ColorPlugin, MeshPickingPlugin, NumberPlugin, ShapesPlugin))
            .add_systems(OnEnter(GameState::Game), (draw_grid, draw_grid_background))
            .insert_resource(Grid::new());
    }
}

#[derive(Resource)]
pub struct Grid{
    pub cells: HashMap<HexCoordinates, CellState>,
    pub colors: Vec<Color>,
    pub number_indicators: Vec<NumberIndicator>,
    pub shapes: Vec<Shape>
}
impl Grid {
    pub fn new() -> Self {
        let radius = HEXAGONS_FROM_CENTER as i8 + 1; // +1 for indicators
        let mut cells = HashMap::new();
        let mut number_coords = Vec::new();

        for q in -radius..=radius {
            let r_min = (-radius).max(-q - radius);
            let r_max = radius.min(-q + radius);
            for r in r_min..=r_max {
                let s = -q - r;
                if q.abs() == 4 || r.abs() == 4 || s.abs() == 4 {
                    number_coords.push((q, r));
                } else {
                    cells.insert(HexCoordinates { q, r }, CellState::Empty);
                }
            }
        }

        let mut color_index = Vec::new();
        let mut colors = Vec::new();
        for _ in 0..3 {
            let mut num = rand::rng().random_range(0..ColorList::COUNT - 1);
            while color_index.contains(&num) {
                num = rand::rng().random_range(0..ColorList::COUNT - 1);
            }
            color_index.push(num);
            colors.push(ColorList::color_index(num));
        }

        let mut number_indicators = Vec::new();
        for coord in number_coords {
            let q = coord.0;
            let r = coord.1;
            let s = -q - r;
            if (q == -4 && r==0) || (q == 4 && r == -4) || (q == 0 && r == 4) {
                continue;
            }
            let hex_coords = HexCoordinates { q, r };
            let new_indicator = if r <= 0 && s >= 0 {
                NumberIndicator::new(vec![1, 2, 3, 1], hex_coords, colors[0])
            } else if r >= 0 && q <= 0 {
                NumberIndicator::new(vec![1, 2, 3, 1], hex_coords, colors[1])
            } else {
                NumberIndicator::new(vec![1, 2, 3], hex_coords, colors[2])
            };
            number_indicators.push(new_indicator)
        }

        let mut shape_index = Vec::new();
        let mut shapes = Vec::new();
        for i in 0..3 {
            let mut num = rand::rng().random_range(0..ShapeList::COUNT);
            while shape_index.contains(&num) {
                num = rand::rng().random_range(0..ShapeList::COUNT);
            }
            let color = if i == 0 || i == 1 {
                colors[0]
            } else {
                colors[1]
            };
            shape_index.push(num);
            shapes.push(Shape::new(ShapeList::shape_index(num), color));
        }


        Grid { cells, colors, number_indicators, shapes }
    }
}


#[derive(Component)]
pub struct GridComponent;

#[derive(Component, Clone, PartialEq, Debug, Copy)]
pub enum CellState{
    Empty,
    Filled(Color),
}

fn draw_grid(
    mut commands: Commands,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    // https://www.redblobgames.com/grids/hexagons/#basics
    let size = HEXAGON_SIZE / 2.0;

    for cell in &grid.cells {
        let coord = cell.0;
        let color = match cell.1 {
            CellState::Empty => Color::srgb(1.0, 1.0, 1.0),
            CellState::Filled(color) => *color // if drawing an already filled grid in the future
        };

        let (x, y) = flat_hex_to_pixel(*coord, size);
        let hex = hexagon_mesh(size - 5.0);
        commands.spawn((
            GridComponent,
            Mesh2d(meshes.add(hex)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, -y, -10.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            *coord,
        )).observe(click_hexagon);
    }
}

fn draw_grid_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let background_hex = hexagon_mesh(GRID_SIZE + 10.0);
    commands.spawn((
        GridComponent,
        Mesh2d(meshes.add(background_hex)),
        MeshMaterial2d(materials.add(Color::srgba(0.678, 0.847, 0.902, 0.5))),
        Transform::from_xyz(0.0, 0.0, -13.0)
        ));
}



fn click_hexagon(
    pressed: On<Pointer<Press>>,
    mut grid: ResMut<Grid>,
    current_color: Res<CurrentColor>,
    hexagons: Query<(&HexCoordinates, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if pressed.button != PointerButton::Primary {
        return;
    }
    // Checks if the query is good and assigns values from query
    let Ok((coord, mesh_material)) = hexagons.get(pressed.event_target()) else {
        return;
    };

    // Unpacks the cell_state from the hashmap if the coord exists in the hashmap
    if let Some(cell_state) = grid.cells.get_mut(coord) {
        *cell_state = match cell_state { // Matches the current state to change color
            CellState::Filled(color) if *color == current_color.color => CellState::Empty, // Matches to the
            _ => CellState::Filled(current_color.color),
        };

        // Finds the new color that was assigned to the cell_state
        let new_color = match *cell_state {
            CellState::Empty => Color::srgb(1.0, 1.0, 1.0),
            CellState::Filled(color) => color,
        }; // Recolors the hexagon
        if let Some(mut material) = materials.get_mut(&mesh_material.0) {
            material.color = new_color;
        }
    }
}

