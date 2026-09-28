use bevy::prelude::Color;
use strum_macros::{EnumCount, EnumIter};


#[derive(EnumCount, EnumIter)]
pub enum ColorList {
    RED,
    YELLOW,
    GREEN,
    BLUE,
    PURPLE,
    WHITE,
    CORRECT,
}

impl ColorList {
    pub fn color(&self) -> Color {
        match self {
            ColorList::RED => Color::srgb(1.0, 0.0, 0.0),
            ColorList::YELLOW => Color::srgb(1.0, 1.0, 0.0),
            ColorList::GREEN => Color::srgb(0.0, 1.0, 0.0),
            ColorList::BLUE => Color::srgb(0.0, 0.0, 1.0),
            ColorList::PURPLE => Color::srgb(1.0, 0.0, 1.0),
            ColorList::WHITE => Color::srgb(1.0, 1.0, 1.0),
            ColorList::CORRECT => Color::srgb(0.0, 0.9, 0.423),
        }
    }

    pub fn color_index(index: usize) -> Color {
        match index {
            0 => Color::srgb(1.0, 0.0, 0.0),
            1 => Color::srgb(1.0, 1.0, 0.0),
            2 => Color::srgb(0.0, 1.0, 0.0),
            3 => Color::srgb(0.0, 0.0, 1.0),
            4 => Color::srgb(1.0, 0.0, 1.0),
            5 => Color::srgb(1.0, 1.0, 1.0),
            _ => Color::srgb(0.0, 0.0, 0.0)
        }
    }
}