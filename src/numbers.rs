use std::f32::consts::PI;
use bevy::math::ops::sqrt;
use bevy::prelude::*;
use crate::grid::{Grid, HEXAGON_SIZE};

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
    coords: (i8, i8), // In qrs coords
    color: Color,
    border: IndicatorBorder
}

impl NumberIndicator {
    pub fn new(numbers: Vec<i8>, coords: (i8, i8), color: Color) -> Self {
        NumberIndicator {
            numbers, coords, color, border: IndicatorBorder::Empty
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
        let mut string = String::new();
        for i in 0..indicator.numbers.len() {
            if i < indicator.numbers.len() - 1 {
                string.push_str(&format!("{}\n", &indicator.numbers[i].abs()));
            } else {
                string.push_str(&format!("{}", &indicator.numbers[i].abs()));
            }
        }
        let indicator_size = Vec2::new(40.0, 60.0);
        let size = HEXAGON_SIZE / 2.0;
        let x = size * 3.0 / 2.0 * indicator.coords.0 as f32;
        let y = -size * (sqrt(3.0) / 2.0 * indicator.coords.0 as f32 + sqrt(3.0) * indicator.coords.1 as f32);
        let s = -indicator.coords.0 - indicator.coords.1;
        let rotate_in_radians = if indicator.coords.1  <= 0 && s >= 0 {
            0.0
        } else if indicator.coords.1 >= 0 && indicator.coords.0 <= 0 {
            - PI / 3.0
        } else {
            PI / 3.0
        };
    commands.spawn((
        Sprite::from_color(indicator.color, indicator_size),
        Transform::from_xyz(x, y, -11.0).with_rotation(Quat::from_rotation_z(rotate_in_radians)),
        children![(
            Text2d::new(&string),
            TextLayout::new(Justify::Center, LineBreak::AnyCharacter),
            Transform::from_translation(Vec3::Z).with_rotation(Quat::from_rotation_z(-rotate_in_radians)),
        )]
        ));
    }
}