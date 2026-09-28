use bevy::prelude::*;
use strum_macros::{EnumCount, EnumIter};
use crate::game::{Game, GridComponent};
use crate::hexagon::{HexCoordinates, HEXAGON_SIZE, hexagon_mesh, pointy_hex_to_pixel};

use super::{GameState};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), draw_shapes);
    }
}

#[derive(EnumCount, EnumIter)]
pub enum ShapeType {
    Triangle,
    LineThree,
    LineThreePlusDot,
    LineFour,
    Diamond,
}

impl ShapeType {
    pub fn shape_index(index: usize) -> ShapeType {
        match index {
            0 => ShapeType::Triangle,
            1 => ShapeType::LineThree,
            2 => ShapeType::LineThreePlusDot,
            3 => ShapeType::LineFour,
            4 => ShapeType::Diamond,
            _ => ShapeType::Triangle,
        }
    }
}

#[derive(Component)]
pub struct ShapeIndicator {
    hexagons: Vec<HexCoordinates>,
    color: Color,
    shape_type: ShapeType,
}

impl ShapeIndicator {
    fn get_hexagons(shape_type: &ShapeType) -> Vec<HexCoordinates> {
        match shape_type {
            ShapeType::Triangle => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:-1}, HexCoordinates{q:1, r:0}],
            ShapeType::LineThree => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}],
            ShapeType::LineThreePlusDot => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}, HexCoordinates{q:1, r:-1}],
            ShapeType::LineFour => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:2, r:0}, HexCoordinates{q:3, r:0}],
            ShapeType::Diamond => vec![HexCoordinates{q:0, r:0}, HexCoordinates{q:1, r:0}, HexCoordinates{q:1, r:-1}, HexCoordinates{q:2, r:-1}],
        }
    }
    pub fn new(shape_type: ShapeType, color: Color) -> Self {
        let hexagons = Self::get_hexagons(&shape_type);
        ShapeIndicator { hexagons, color, shape_type }
    }

    fn rotate_cw(shape: Vec<HexCoordinates>) -> Vec<HexCoordinates> {
        let mut new_shape = Vec::new();
        for hexagon in shape {
            let s = -hexagon.q - hexagon.r;
            new_shape.push(HexCoordinates{q: -hexagon.r, r: -s});
        }
        new_shape
    }

    pub fn rotate_cw_n_times(shape_type: &ShapeType, times: i32) -> Vec<HexCoordinates> {
        let mut new_shape = Self::get_hexagons(&shape_type);
        for _ in 0..times {
            new_shape = Self::rotate_cw(new_shape);
        }
        new_shape
    }
}


fn draw_shapes(
    mut commands: Commands,
    game: Res<Game>,
    windows: Query<&mut Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let shapes = &game.shape_indicators;
    let size = HEXAGON_SIZE / 3.0;
    let y = -windows.single().unwrap().resolution.height() / 2.0 + HEXAGON_SIZE;
    let x = -windows.single().unwrap().resolution.width() / 2.0 + HEXAGON_SIZE / 2.0;

    let mut shape_offset = 0.0;
    for current_shape in shapes {
        for hexagon in &current_shape.hexagons {
            let (x_local, y_local) = pointy_hex_to_pixel(*hexagon, size);
            let hex = hexagon_mesh(size - 1.0);
            commands.spawn((
                DespawnOnExit(GameState::Game),
                GridComponent,
                Mesh2d(meshes.add(hex)),
                MeshMaterial2d(materials.add(current_shape.color)),
                Transform::from_xyz(x + x_local, y + -y_local + shape_offset, -10.0),
            ));
        }

        shape_offset += match current_shape.shape_type {
            ShapeType::LineFour | ShapeType::LineThree => size * 2.25,
            _ => size * 3.25
        };
    }
}

