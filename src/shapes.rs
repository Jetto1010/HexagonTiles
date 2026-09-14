use bevy::math::ops::sqrt;
use bevy::prelude::*;
use strum_macros::{EnumCount, EnumIter};
use crate::grid::{Grid, HexCoordinates, HEXAGON_SIZE};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_shapes);
    }
}

#[derive(EnumCount, EnumIter)]
pub enum ShapeList{
    Triangle,
    LineThree,
    LineThreePlusDot,
    LineFour,
    Diamond,
}

impl ShapeList{
    #[allow(dead_code)]
    pub fn shape(&self) -> Vec<HexCoordinates> { // These are in pointy-top hex coordinates. Does not include (0, 0, 0)
        match self {
            ShapeList::Triangle => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:0, r:1}, HexCoordinates{q:0, s:-1, r:1}],
            ShapeList::LineThree => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}],
            ShapeList::LineThreePlusDot => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:1, s:0, r:-1}],
            ShapeList::LineFour => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:2, s:-2, r:0}],
            ShapeList::Diamond => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:-1, s:0, r:1}, HexCoordinates{q:0, s:-1, r:1}],
        }
    }

    pub fn shape_index(index: usize) -> Vec<HexCoordinates> {
        match index {
            0 => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:0, r:1}, HexCoordinates{q:0, s:-1, r:1}],
            1 => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}],
            2 => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:1, s:0, r:-1}],
            3 => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:-1, s:1, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:2, s:-2, r:0}],
            4 => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:1, s:-1, r:0}, HexCoordinates{q:-1, s:0, r:1}, HexCoordinates{q:0, s:-1, r:1}],
            _ => vec![HexCoordinates{q:0, s:0, r:0}, HexCoordinates{q:0, s:0, r:0}],
        }
    }
}

fn draw_shapes(
    mut commands: Commands,
    grid: Res<Grid>,
    windows: Query<&mut Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let shapes = &grid.shapes;
    let size = HEXAGON_SIZE / 3.0;
    let y = -windows.single().unwrap().resolution.height() / 2.0 + HEXAGON_SIZE;
    let x = -windows.single().unwrap().resolution.width() / 2.0 + HEXAGON_SIZE;

    for i in 0..shapes.len() {
        let color = if i == 0 || i == 1 {
            grid.colors[0]
        } else {
            grid.colors[1]
        };
        let shape = &shapes[i];

        let shape_offset = if shape == &ShapeList::shape(&ShapeList::LineFour) || shape == &ShapeList::shape(&ShapeList::LineThree) {
            size * i as f32 * 1.5
        } else {
            size * i as f32 * 3.5
        };

        for hexagon in shape {
            let x_local = size * (sqrt(3.0) * hexagon.q as f32 + sqrt(3.0) / 2.0 * hexagon.r as f32);
            let y_local = size * 3.0 / 2.0 * hexagon.r as f32 + shape_offset; // Plus offset per shape
            let hex = crate::grid::hexagon_mesh(size);
            commands.spawn((
                Mesh2d(meshes.add(hex)),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x + x_local, y + y_local, -10.0),
            ));
        }
    }
}

