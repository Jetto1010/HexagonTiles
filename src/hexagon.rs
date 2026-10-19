use std::f32::consts::PI;
use bevy::asset::RenderAssetUsages;
use bevy::math::ops::sqrt;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use crate::game::{GRID_SIZE, HEXAGONS_FROM_CENTER, RADIUS};

pub const HEXAGON_SIZE: f32 = GRID_SIZE / HEXAGONS_FROM_CENTER; // Flat-top -> size from center to right edge


#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct HexCoordinates {
    pub q: i8,
    pub r: i8,
}

const DIRECTIONS: [(i8, i8); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

impl HexCoordinates {
    pub fn get_neighbours(self) -> Vec<HexCoordinates> {
        let mut neighbors = Vec::new();
        for direction in DIRECTIONS.iter() {
            let (q, r) = (self.q + direction.0, self.r + direction.1);
            if !(q.abs().max(r.abs()).max((-q -r).abs()) >= RADIUS) {
                neighbors.push(HexCoordinates { q, r });
            }
        }

        neighbors
    }
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
    let mut indices = vec![0, 1, 6];
    for i in 2..7 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    // Sets indices
    hex.insert_indices(Indices::U16(indices));
    hex
}

pub fn flat_hex_to_pixel(hexagon: HexCoordinates, size: f32) -> (f32, f32) {
    let x = size * 3.0 / 2.0 * hexagon.q as f32;
    let y = size * (sqrt(3.0) / 2.0 * hexagon.q as f32 + sqrt(3.0) * hexagon.r as f32);
    (x, y)
}

pub fn pointy_hex_to_pixel(hexagon: HexCoordinates, size: f32) -> (f32, f32) {
    let x = size * (sqrt(3.0) * hexagon.q as f32 + sqrt(3.0) / 2.0 * hexagon.r as f32);
    let y = size * 3.0 / 2.0 * hexagon.r as f32;
    (x, y)
}