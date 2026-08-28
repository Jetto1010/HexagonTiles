use bevy::math::ops::sqrt;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app
            .insert_resource(Grid::new(4))
            .add_systems(Startup, draw_grid);
    }
}


#[derive(Resource)]
pub struct Grid{
    pub cells: HashMap<HexCoordinates, CellState>,
}
impl Grid {
    pub fn new(radius: i8) -> Self {
        let mut cells = HashMap::new();

        for q in -radius..=radius {
            let r_min = (-radius).max(-q - radius);
            let r_max = radius.min(-q + radius);
            for r in r_min..=r_max {
                let s = -q - r;
                cells.insert(HexCoordinates{q, r, s}, CellState::Empty);
            }
        }

        Grid { cells }
    }
}


#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct HexCoordinates {
    pub q: i8,
    pub s: i8,
    pub r: i8,
}

#[derive(Component, Clone, PartialEq, Debug, Copy)]
pub enum CellState{
    Empty,
    Filled(Color),
}

pub fn draw_grid(
    mut commands: Commands,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    for cell in &grid.cells {
        let coord = cell.0;
        let color = match cell.1 {
            CellState::Empty => Color::srgb(1.0, 1.0, 1.0),
            CellState::Filled(color) => *color
        };

        let x = coord.q as f32 * sqrt(3.0) + coord.r as f32 * sqrt(3.0) / 2.0;
        let y = 3.0 * coord.r as f32 / 2.0;

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x * 50.0, y * 50.0, -20.0).with_scale(Vec3::splat(10.0)),
        ));
    }
}