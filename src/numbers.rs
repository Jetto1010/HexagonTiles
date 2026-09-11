use bevy::math::ops::sqrt;
use bevy::prelude::*;
use bevy::text::TextBounds;
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
                string.push_str(&format!("{}\n", &indicator.numbers[i]));
            } else {
                string.push_str(&format!("{}", &indicator.numbers[i]));
            }
        }
        let indicator_size = Vec2::new(5.0, 5.0);
        let size = HEXAGON_SIZE / 2.0;
        let x = size * 3.0 / 2.0 * indicator.coords.0 as f32;
        let y = size * (sqrt(3.0) / 2.0 * indicator.coords.0 as f32 + sqrt(3.0) * indicator.coords.1 as f32);
    commands.spawn((
        Sprite::from_color(indicator.color, indicator_size),
        Transform::from_xyz(x, y, -11.0),
        children![
            Text2d::new(&string),
            TextLayout::new(Justify::Center, LineBreak::AnyCharacter),
            TextBounds::from(indicator_size),
            Transform::from_translation(Vec3::Z)
        ]
        ));
    }
}