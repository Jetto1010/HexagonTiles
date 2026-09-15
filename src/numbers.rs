use std::f32::consts::PI;
use bevy::prelude::*;
use crate::grid::{Grid};
use crate::hexagon::{flat_hex_to_pixel, HexCoordinates, HEXAGON_SIZE};

pub struct NumberPlugin;

impl Plugin for NumberPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_indicators);
    }
}


#[derive(Component)]
pub struct NumberIndicator {
    numbers: Vec<i8>,
    hex_coords: HexCoordinates, // In qrs coords
    color: Color,
    border: IndicatorBorder
}

impl NumberIndicator {
    pub fn new(numbers: Vec<i8>, hex_coords: HexCoordinates, color: Color) -> Self {
        NumberIndicator {
            numbers, hex_coords, color, border: IndicatorBorder::Empty
        }
    }
}

#[derive(Component)]
pub enum IndicatorBorder {
    Empty,
    Filled(bool) // Whether the border is to display correct or wrong colors
}


fn draw_indicators(
    mut commands: Commands,
    grid: Res<Grid>,
){
    let number_indicator = &grid.number_indicators;
    for indicator in number_indicator.iter() {
        let size = HEXAGON_SIZE / 2.0;
        let (x, y) = flat_hex_to_pixel(indicator.hex_coords, size);
        let indicator_size = Vec2::new(40.0, 70.0);
        let s = -indicator.hex_coords.q - indicator.hex_coords.r;
        let rotate_in_radians = if indicator.hex_coords.r  <= 0 && s >= 0 {
            0.0
        } else if indicator.hex_coords.r >= 0 && indicator.hex_coords.q <= 0 {
            - PI / 3.0
        } else {
            PI / 3.0
        };
        let number_positions = if indicator.numbers.len() == 1 {
            vec![0.0]
        } else if indicator.numbers.len() == 2 {
            vec![-0.67, 0.67]
        } else if indicator.numbers.len() == 3 {
            vec![-0.67, 0.0, 0.67]
        } else {
            vec![-0.67, -0.22, 0.22, 0.67]
        };
    commands
        .spawn((
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