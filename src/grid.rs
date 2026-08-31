use std::f32::consts::PI;
use bevy::asset::RenderAssetUsages;
use bevy::math::ops::sqrt;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app
            .insert_resource(Grid::new(3))
            .add_systems(Startup, draw_grid);
    }
}

const GRID_SIZE: f32 = 500.0;
const RADIUS_OF_GRID : f32 = 3.0;

#[derive(Resource)]
pub struct Grid{
    pub cells: HashMap<HexCoordinates, CellState>,
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
    // https://www.redblobgames.com/grids/hexagons/#basics
    let hex_height = GRID_SIZE / (RADIUS_OF_GRID * 2.0 - 1.0);
    let size = hex_height / 2.0;

    for cell in &grid.cells {
        let coord = cell.0;
        let color = match cell.1 {
            CellState::Empty => Color::srgb(1.0, 1.0, 1.0),
            CellState::Filled(color) => *color
        };

        let y = size * 3.0 / 2.0 * coord.q as f32;
        let x = size * (sqrt(3.0) / 2.0 * coord.q as f32 + sqrt(3.0) * coord.r as f32);
        let hex = hexagon(0.5);
        commands.spawn((
            Mesh2d(meshes.add(hex)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y, -10.0).with_scale(Vec3::splat(hex_height - 10.0)),
        ));
    }
}

fn hexagon(radius: f32) -> Mesh {
    let mut hex = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..6 {
        let angle = i as f32 * PI / 3.0;
        v_pos.push([radius * ops::sin(angle), radius * ops::cos(angle), 0.0]);
    }
    // Sets position attribut
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