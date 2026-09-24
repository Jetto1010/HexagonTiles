use std::f32::consts::PI;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::color_list::ColorList;
use crate::color_selection::{ColorPlugin, CurrentColor};
use crate::numbers::{NumberPlugin, NumberIndicator};
use crate::shapes::{ShapesPlugin, ShapeIndicator, ShapeType};
use crate::hexagon::{HexCoordinates, HEXAGON_SIZE, hexagon_mesh, flat_hex_to_pixel};
use rand::{RngExt, SeedableRng};
use rand::rngs::ChaCha8Rng;
use rand::seq::SliceRandom;
use strum::{EnumCount};

use super::{GameState};

pub const GRID_SIZE: f32 = 250.0; // Pointy-top -> size from center to top
pub const HEXAGONS_FROM_CENTER: f32 = 3.0;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
            .add_plugins((ColorPlugin, MeshPickingPlugin, NumberPlugin, ShapesPlugin))
            .add_systems(OnEnter(GameState::Game), (draw_grid, draw_grid_background));
    }
}

#[derive(Resource)]
pub struct Game {
    pub cells: HashMap<HexCoordinates, CellState>,
    pub colors: Vec<Color>,
    pub shape_indicators: Vec<ShapeIndicator>,
    pub number_indicators: Vec<NumberIndicator>,
}

pub fn new_game(seed: String) -> Game {
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

    let mut rng = ChaCha8Rng::seed_from_u64(seahash::hash(seed.as_bytes()));
    let mut color_index = Vec::new();
    let mut colors = Vec::new();
    for _ in 0..3 {
        let mut num = rng.random_range(0..ColorList::COUNT - 1);
        while color_index.contains(&num) {
            num = rng.random_range(0..ColorList::COUNT - 1);
        }
        color_index.push(num);
        colors.push(ColorList::color_index(num));
    }

    let mut shape_index = Vec::new();
    let mut shape_indicators = Vec::new();
    for i in 0..3 {
        let mut num = rng.random_range(0..ShapeType::COUNT);
        while shape_index.contains(&num) {
            num = rng.random_range(0..ShapeType::COUNT);
        }
        let color = if i == 0 || i == 1 {
            colors[0]
        } else {
            colors[1]
        };
        shape_index.push(num);
        shape_indicators.push(ShapeIndicator::new(ShapeType::shape_index(num), color));
    }

    // Placing the shape in the puzzle
    let mut puzzle = cells.clone();
    let mut coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
    coords.shuffle(&mut rng); // Gets the coords and then shuffles them in random order
    let mut placed_first_shape = 0;
    for i in 0..3 {
        let current_shape_type = ShapeType::shape_index(shape_index[i]);
        let (mut min, mut max) = match current_shape_type {
            ShapeType::Triangle => (3, 6),
            _ => (1, 3)
        };
        if i == 1 { // Logic for the second shape of the first color -> shape might not be on the grid
            if max - placed_first_shape <= 0 {continue}
            max = max - placed_first_shape;
            min = min - placed_first_shape;
        }

        let n_to_place =  rng.random_range(min..=max);
        if n_to_place <= 0 {continue} // Second shape
        if i == 0 {placed_first_shape = n_to_place } // Number placed for the first color
        let mut n_placed = 0;
        while n_placed < n_to_place && !coords.is_empty(){
            let mut placed = false;
            let Some(random_coord) = coords.pop() else { break };
            if let Some(state) = puzzle.get(&random_coord) {
                if *state != CellState::Empty { continue } // Continue if filled
                let mut rotation = [0, 1, 2, 3, 4, 5]; // Random rotation of place shape
                rotation.shuffle(&mut rng);

                for j in rotation {
                    if placed {break}

                    let rotated_shape = ShapeIndicator::rotate_cw_n_times(&current_shape_type, j);
                    let mut current_shape = vec!(random_coord);
                    for k in 1..rotated_shape.len() {
                        if placed {break}

                        let (q, r) = (random_coord.q + rotated_shape[k].q, random_coord.r + rotated_shape[k].r);
                        let cur_hexagon = HexCoordinates { q, r };
                        if let Some(new_state) = puzzle.get(&cur_hexagon) {
                            if *new_state != CellState::Empty { break }

                            current_shape.push(cur_hexagon);
                            if k == rotated_shape.len() - 1 {
                                let color = if i == 0 || i == 1 {
                                    colors[0]
                                } else {
                                    colors[1]
                                };
                                for placed_hexagon in &current_shape {
                                    puzzle.insert(*placed_hexagon, CellState::Filled(color));
                                }
                                n_placed = n_placed + 1;
                                placed = true
                            }
                        } else { break } // If invalid coord
                    }
                }
            }
        }
    }

    let fill_coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
    for fillable in fill_coords {
        if let Some(state) = puzzle.get(&fillable) {
            if *state == CellState::Empty {
                puzzle.insert(fillable, CellState::Filled(colors[2]));
            }
        }
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
        let new_indicator = if r <= 0 && s >= 0 { // Along q
            let numbers = NumberIndicator::count_color_along_axis(&puzzle, 0, hex_coords, colors[0]);
            NumberIndicator::new(numbers, hex_coords, colors[0], 0)
        } else if r >= 0 && q <= 0 { // Along s
            let numbers = NumberIndicator::count_color_along_axis(&puzzle, 1, hex_coords, colors[1]);
            NumberIndicator::new(numbers, hex_coords, colors[1], 1)
        } else { // Along r
            let numbers = NumberIndicator::count_color_along_axis(&puzzle, 2, hex_coords, colors[2]);
            NumberIndicator::new(numbers, hex_coords, colors[2], 2)
        };
        number_indicators.push(new_indicator)
    }

    Game { cells, colors, shape_indicators, number_indicators }
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
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    // https://www.redblobgames.com/grids/hexagons/#basics
    let size = HEXAGON_SIZE / 2.0;

    for cell in &game.cells {
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
    mut game: ResMut<Game>,
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
    if let Some(cell_state) = game.cells.get_mut(coord) {
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

