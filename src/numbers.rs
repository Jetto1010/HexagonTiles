use std::f32::consts::PI;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::color_list::ColorList;
use crate::game::{CellState, Game, GridComponent, ColoredHexagonEvent};
use crate::hexagon::{flat_hex_to_pixel, HexCoordinates, HEXAGON_SIZE};

use super::{GameState};

const INDICATOR_SIZE: Vec2 = Vec2::new(40.0, 70.0);


pub struct NumberPlugin;

impl Plugin for NumberPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), draw_indicators)
            .add_observer(check_indicator_on_hexagon_updates);
    }
}


#[derive(Component)]
pub struct NumberIndicator {
    numbers: Vec<i8>,
    hexagon_coords: HexCoordinates, // Position
    color: Color,
    axis: u8, // 0 = Q, 1 = S, 2 = R
    pub border: Option<NumberIndicatorBorder>
}

#[derive(Component)]
pub struct NumberIndicatorBorder(pub Entity);


impl NumberIndicator {
    pub fn new(numbers: Vec<i8>, hex_coords: HexCoordinates, color: Color, axis: u8) -> Self {
        NumberIndicator { numbers, hexagon_coords: hex_coords, color, axis, border: None}
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

        if axis == 0 {
            numbers
        } else {
            numbers.reverse();
            numbers
        }
    }

    pub fn remove_border(number_indicator: &mut NumberIndicator, commands: &mut Commands) {
        if let Some(border) = number_indicator.border.take() {
            commands.entity(border.0).despawn();
            number_indicator.border = None;
        }
    }
}

fn draw_indicators(
    mut commands: Commands,
    game: Res<Game>,
){
    let number_indicator = &game.number_indicators;
    for indicator in number_indicator.iter() {
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
        .spawn(get_indicator_sprite(false, indicator.hexagon_coords, indicator.axis, indicator.color))
        .with_children(|parent| {
            for i in 0..number_positions.len() {
                let child_y = -number_positions[i] * INDICATOR_SIZE.y / 2.0;
                let string = String::from(&indicator.numbers[i].to_string());
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

fn check_indicator_on_hexagon_updates(
    colored_hexagon_event: On<ColoredHexagonEvent>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let coords = colored_hexagon_event.hexagon_coords;
    let cells = game.cells.clone();
    for indicator in game.number_indicators.iter_mut() {
        if (indicator.axis == 0 && indicator.hexagon_coords.q == coords.q) || // Along q
            (indicator.axis == 1 && -indicator.hexagon_coords.q - indicator.hexagon_coords.r == -coords.q - coords.r) || // Along s
            (indicator.axis == 2 && indicator.hexagon_coords.r == coords.r){ // Along r
            let current_numbers = NumberIndicator::count_color_along_axis(&cells, indicator.axis, indicator.hexagon_coords, indicator.color);

            if current_numbers == indicator.numbers {
                if indicator.border.is_none() {
                    let border = commands
                        .spawn(
                            get_indicator_sprite(true, indicator.hexagon_coords, indicator.axis, ColorList::color(&ColorList::CORRECT)),
                        ).id();
                    indicator.border = Some(NumberIndicatorBorder(border));
                }
            } else if let Some(indicator_border) = indicator.border.take() {
                commands.entity(indicator_border.0).despawn();
            }
        }
    }

    let mut is_correct = true;
    for indicator in game.number_indicators.iter() {
        if indicator.border.is_none() {
            is_correct = false;
            break
        }
    }

    if is_correct {
        println!("All correct")
    }
}

fn get_indicator_sprite(border: bool, hexagon_coords: HexCoordinates, axis: u8, color: Color) -> impl Bundle {
    let (x, y) = flat_hex_to_pixel(hexagon_coords, HEXAGON_SIZE / 2.0);
    let size = if border {
        Vec2::new(INDICATOR_SIZE.x + 8.0, INDICATOR_SIZE.y + 8.0)
    } else { INDICATOR_SIZE };
    let rotate_in_radians = if axis == 0 {
        0.0
    } else if axis == 1 {
        -PI / 3.0
    } else {
        PI / 3.0
    };
    let z = if border {
        -12.0
    } else { -11.0 };
    (
        DespawnOnExit(GameState::Game),
        GridComponent,
        Sprite::from_color(color, size),
        Transform::from_xyz(x, -y, z).with_rotation(Quat::from_rotation_z(rotate_in_radians)),
    )
}