use bevy::prelude::*;
use strum_macros::{EnumCount, EnumIter};
use crate::grid::{Grid};
use crate::hexagon::{HexCoordinates, HEXAGON_SIZE, hexagon_mesh, pointy_hex_to_pixel};

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
    pub fn shape_index(index: usize) -> ShapeList {
        match index {
            0 => ShapeList::Triangle,
            1 => ShapeList::LineThree,
            2 => ShapeList::LineThreePlusDot,
            3 => ShapeList::LineFour,
            4 => ShapeList::Diamond,
            _ => ShapeList::Triangle,
        }
    }
}

#[derive(Component)]
pub struct Shape {
    hexagons: Vec<HexCoordinates>,
    color: Color,
    shape: ShapeList,
}

impl Shape {
    pub fn new(shape: ShapeList, color: Color) -> Self {
        let hexagons = match shape {
            ShapeList::Triangle => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:-1}, HexCoordinates{q:1, r:0}],
            ShapeList::LineThree => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}],
            ShapeList::LineThreePlusDot => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}, HexCoordinates{q:1, r:-1}],
            ShapeList::LineFour => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}, HexCoordinates{q:3, r:0}],
            ShapeList::Diamond => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:1, r:-1}, HexCoordinates{q:2, r:-1}],
        };

        Shape { hexagons, color, shape }
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
    let x = -windows.single().unwrap().resolution.width() / 2.0 + HEXAGON_SIZE / 2.0;

    let mut shape_offset = 0.0;
    for current_shape in shapes {
        for hexagon in &current_shape.hexagons {
            let (x_local, y_local) = pointy_hex_to_pixel(*hexagon, size);
            let hex = hexagon_mesh(size - 1.0);
            commands.spawn((
                Mesh2d(meshes.add(hex)),
                MeshMaterial2d(materials.add(current_shape.color)),
                Transform::from_xyz(x + x_local, y + -y_local + shape_offset, -10.0),
            ));
        }

        shape_offset += match current_shape.shape {
            ShapeList::LineFour | ShapeList::LineThree => size * 2.25,
            _ => size * 3.25
        };
    }
}

