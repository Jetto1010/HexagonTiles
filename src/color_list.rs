use strum_macros::{EnumCount, EnumIter};


#[derive(EnumCount, EnumIter)]
pub enum ColorList {
    RED,
    YELLOW,
    GREEN,
    BLUE,
    PURPLE,
    WHITE,
}

impl ColorList {
    pub fn color(&self) -> (f32, f32, f32) {
        match self {
            ColorList::RED => (1.0, 0.0, 0.0),
            ColorList::YELLOW => (1.0, 1.0, 0.0),
            ColorList::GREEN => (0.0, 0.0, 1.0),
            ColorList::BLUE => (0.0, 0.0, 1.0),
            ColorList::PURPLE => (1.0, 0.0, 1.0),
            ColorList::WHITE => (1.0, 1.0, 1.0),
        }
    }

    pub fn color_index(index: usize) -> (f32, f32, f32) {
        match index {
            0 => (1.0, 0.0, 0.0),
            1 => (1.0, 1.0, 0.0),
            2 => (0.0, 0.0, 1.0),
            3 => (0.0, 0.0, 1.0),
            4 => (1.0, 0.0, 1.0),
            _ => (1.0, 1.0, 1.0),
        }
    }
}