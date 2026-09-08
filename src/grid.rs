use std::f32::consts::PI;
use bevy::asset::RenderAssetUsages;
use bevy::math::ops::sqrt;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::RngExt;
use crate::color_list::ColorList;
use crate::color_selection::{ColorPlugin, CurrentColor};
use strum::{EnumCount};


pub const GRID_SIZE: f32 = 500.0;
pub const HEXAGONS_FROM_CENTER: f32 = 3.0;
pub const HEXAGON_SIZE: f32 = GRID_SIZE / (HEXAGONS_FROM_CENTER * 2.0 - 1.0);

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app
            .add_plugins((ColorPlugin, MeshPickingPlugin))
            .insert_resource(Grid::new(HEXAGONS_FROM_CENTER as i8))
            .add_systems(Startup, (draw_grid, draw_grid_background));
    }
}

#[derive(Resource)]
pub struct Grid{
    pub cells: HashMap<HexCoordinates, CellState>,
    pub colors: Vec<Color>,
}
impl Grid {
    pub fn new(radius: i8) -> Self {
        // Check if < 1 radius
        let mut cells = HashMap::new();

        for q in -radius..=radius {
            let r_min = (-radius).max(-q - radius);
            let r_max = radius.min(-q + radius);
            for r in r_min..=r_max {
                let s = -q - r;
                cells.insert(HexCoordinates{q, r, s}, CellState::Empty);
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

        Grid { cells, colors }
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

        let x = size * 3.0 / 2.0 * coord.q as f32;
        let y = size * (sqrt(3.0) / 2.0 * coord.q as f32 + sqrt(3.0) * coord.r as f32);
        let hex = hexagon_mesh(size - 10.0);
        commands.spawn((
            Mesh2d(meshes.add(hex)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y, -10.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            *coord,
        )).observe(click_hexagon);
    }
}

fn draw_grid_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let background_hex = hexagon_mesh(GRID_SIZE / 2.0 + HEXAGON_SIZE / 2.0);
    commands.spawn((
        Mesh2d(meshes.add(background_hex)),
        MeshMaterial2d(materials.add(Color::srgba(0.678, 0.847, 0.902, 0.5))),
        Transform::from_translation(Vec3::Z - 13.0)
        ));
}

pub fn hexagon_mesh(size: f32) -> Mesh {
    let mut hex = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::all(),
    );
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..6 {
        let angle = i as f32 * PI / 3.0;
        v_pos.push([size * ops::sin(angle), size * ops::cos(angle), 0.0]);
    }

    hex.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // 0, 1, 6 is last triangle
    let mut indicies = vec![0, 1, 6];
    for i in 2..7 {
        indicies.extend_from_slice(&[0, i, i - 1]);
    }
    // Sets indices
    hex.insert_indices(Indices::U16(indicies));
    hex
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

