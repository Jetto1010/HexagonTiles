use std::f32::consts::PI;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::game::{CellState, Game, GridComponent};
use crate::hexagon::{flat_hex_to_pixel, HexCoordinates, HEXAGON_SIZE};

use super::{GameState};

pub struct NumberPlugin;

impl Plugin for NumberPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), draw_indicators);
    }
}


#[derive(Component)]
pub struct NumberIndicator {
    numbers: Vec<i8>,
    hex_coords: HexCoordinates, // Position
    color: Color,
    axis: u8, // 0 = Q, 1 = S, 2 = R
    border: IndicatorBorder
}

impl NumberIndicator {
    pub fn new(numbers: Vec<i8>, hex_coords: HexCoordinates, color: Color, axis: u8) -> Self {
        NumberIndicator { numbers, hex_coords, color, axis, border: IndicatorBorder::Empty }
    }

    pub fn count_color_along_axis(cells: &HashMap<HexCoordinates, CellState>, axis: u8, start_coord: HexCoordinates, color: Color) -> Vec<i8> {
        let mut numbers = Vec::new();
        let (dq, dr) = if axis == 0 {
            (0, 1)
        } else if axis == 1 {
            (1, -1)
        } else {
            (-1, 0)
        };

        let mut q = start_coord.q + dq;
        let mut r = start_coord.r + dr;
        let mut row = 0;
        while q.abs() < 4 && r.abs() < 4{
            if let Some(state) = cells.get(&HexCoordinates{ q, r }) {
                if *state == CellState::Filled(color) {
                    row += 1;
                } else {
                    if row > 0 {
                        numbers.push(row);
                        row = 0;
                    }
                }

                q = q + dq;
                r = r + dr;
            } else { break; } // Breaks if outside gird
        }
        if row > 0 { // Add rest
            numbers.push(row);
        }
        if numbers.is_empty() { // If no of indicators color
            numbers.push(0);
        }
        numbers
    }
}

#[derive(Component)]
pub enum IndicatorBorder {
    Empty,
    Filled(bool) // Whether the border is to display correct or wrong colors
}


fn draw_indicators(
    mut commands: Commands,
    game: Res<Game>,
){
    let number_indicator = &game.number_indicators;
    for indicator in number_indicator.iter() {
        let size = HEXAGON_SIZE / 2.0;
        let (x, y) = flat_hex_to_pixel(indicator.hex_coords, size);
        let indicator_size = Vec2::new(40.0, 70.0);
        let rotate_in_radians = if indicator.axis == 0 {
            0.0
        } else if indicator.axis == 1 {
            - PI / 3.0
        } else {
            PI / 3.0
        };
        let number_positions = if indicator.numbers.len() == 1 {
            vec![0.0]
        } else if indicator.numbers.len() == 2 {
            vec![-0.47, 0.47]
        } else if indicator.numbers.len() == 3 {
            vec![-0.67, 0.0, 0.67]
        } else {
            vec![-0.67, -0.22, 0.22, 0.67]
        };
    commands
        .spawn((
            GridComponent,
            Sprite::from_color(indicator.color, indicator_size),
            Transform::from_xyz(x, -y, -11.0).with_rotation(Quat::from_rotation_z(rotate_in_radians)),
        ))
        .with_children(|parent| {
            for i in 0..number_positions.len() {
                let child_y = -number_positions[i] * indicator_size.y / 2.0;
                let string = String::from(&indicator.numbers[i].abs().to_string());
                parent.spawn((
                    Text2d(string),
                    TextColor(Color::BLACK),
                    TextLayout::new(Justify::Center, LineBreak::AnyCharacter),
                    Transform::from_translation(Vec3::new(0.0, child_y, 1.0)).with_rotation(Quat::from_rotation_z(-rotate_in_radians)),
                ));
            }
        });
    }
}