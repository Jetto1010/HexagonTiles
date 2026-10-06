use std::f32::consts::PI;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::ecs::observer::ObservedBy;
use crate::color_list::ColorList;
use crate::color_selection::{ColorPlugin, CurrentColor};
use crate::numbers::{NumberPlugin, NumberIndicator};
use crate::shapes::{ShapesPlugin, ShapeIndicator, ShapeType, SHAPE_PAIRS};
use crate::hexagon::{HexCoordinates, HEXAGON_SIZE, hexagon_mesh, flat_hex_to_pixel};
use crate::menu::{BUTTON_COLOR, TEXT_COLOR};
use rand::{RngExt, SeedableRng};
use rand::rngs::ChaCha8Rng;
use rand::seq::SliceRandom;
use strum::{EnumCount};
use super::{GameState};

pub const GRID_SIZE: f32 = 250.0; // Pointy-top -> size from center to top
pub const HEXAGONS_FROM_CENTER: f32 = 3.0;
const CHANCE_SHAPE_PAIR: f64 = 0.6;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
            .add_plugins((ColorPlugin, MeshPickingPlugin, NumberPlugin, ShapesPlugin))
            .add_systems(OnEnter(GameState::Game), (draw_grid, draw_grid_background))
            .add_systems(Update, (reset_colors, exit_to_menu, click_complete_buttons).run_if(in_state(GameState::Game)))
            .add_observer(puzzle_correct_handle.run_if(in_state(GameState::Game)));
    }
}

#[derive(Resource)]
pub struct Game {
    pub cells: HashMap<HexCoordinates, CellState>,
    pub colors: Vec<Color>,
    pub shape_indicators: Vec<ShapeIndicator>,
    pub number_indicators: Vec<NumberIndicator>,
}

impl Game {
    pub fn new(seed: String) -> Self {
        let radius = HEXAGONS_FROM_CENTER as i8 + 1; // +1 for indicators
        let mut cells = HashMap::new();
        let mut number_coords = Vec::new();

        for q in -radius..=radius {
            let r_min = (-radius).max(-q - radius);
            let r_max = radius.min(-q + radius);
            for r in r_min..=r_max {
                let s = -q - r;
                if q.abs() == 4 || r.abs() == 4 || s.abs() == 4 {
                    number_coords.push((q, r));
                } else {
                    cells.insert(HexCoordinates { q, r }, CellState::Empty);
                }
            }
        }

        let mut rng = ChaCha8Rng::seed_from_u64(seahash::hash(seed.as_bytes()));
        let mut color_index = Vec::new();
        let mut colors = Vec::new();
        let color_count = ColorList::COUNT - 2; // Exclude black and white
        for _ in 0..3 {
            let mut num = rng.random_range(0..color_count);
            while color_index.contains(&num) {
                num = rng.random_range(0..color_count);
            }
            color_index.push(num);
            colors.push(ColorList::color_index(num));
        }

        // Placing the shape in the puzzle
        let mut puzzle = cells.clone();
        let mut coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
        let mut shapes: Vec<ShapeType> = Vec::new();
        coords.shuffle(&mut rng); // Gets the coords and then shuffles them in random order
        let use_shape_pair = rng.random_bool(CHANCE_SHAPE_PAIR);
        for i in 0..2 {
            if i == 0 && use_shape_pair { // If first shape and going to be two shapes
                let n_shape_pairs = SHAPE_PAIRS.len();
                let shape_pair = &SHAPE_PAIRS[rng.random_range(0..n_shape_pairs)];
                shapes.push(shape_pair.0);
                shapes.push(shape_pair.1);
                let min = 3;
                let max = match shape_pair.0 {
                    ShapeType::Triangle => 5,
                    _ => 4
                };
                let total_shapes_to_place = rng.random_range(min..=max);
                let first_shapes_to_place = rng.random_range(0..=total_shapes_to_place);

                let randomized_coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
                let number_placed_first = place_shape(&mut puzzle, randomized_coords, shape_pair.0, first_shapes_to_place, &mut rng, colors[0]);

                if total_shapes_to_place - number_placed_first > 0 {
                    let randomized_coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
                    place_shape(&mut puzzle, randomized_coords, shape_pair.1, total_shapes_to_place - number_placed_first, &mut rng, colors[0]);
                }
            } else {
                let mut shape_type_numbers = Vec::new();
                for i in 0..ShapeType::COUNT {shape_type_numbers.push(i);}
                shape_type_numbers.shuffle(&mut rng);

                let shape = shape_type_numbers.into_iter().map(ShapeType::shape_index).find(|s| !shapes.contains(s)).expect("guaranteed shape");
                shapes.push(shape);
                let min = 2;
                let max = match shape {
                    ShapeType::Triangle => 5,
                    ShapeType::Curve => 5,
                    ShapeType::LineThree => 4,
                    ShapeType::Diamond => 4,
                    _ => 3
                };
                let total_shapes_to_place = rng.random_range(min..=max);
                let randomized_coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
                place_shape(&mut puzzle, randomized_coords, shape, total_shapes_to_place, &mut rng, colors[i]);
            }
        }


        let mut shape_indicators = Vec::new();
        shape_indicators.push(ShapeIndicator::new(shapes[0], colors[0]));
        if shapes.len() == 2 {
            shape_indicators.push(ShapeIndicator::new(shapes[1], colors[1]));
        } else {
            shape_indicators.push(ShapeIndicator::new(shapes[1], colors[0]));
            shape_indicators.push(ShapeIndicator::new(shapes[2], colors[1]));
        }

        let fill_coords: Vec<HexCoordinates> = puzzle.keys().cloned().collect();
        for fillable in fill_coords {
            if let Some(state) = puzzle.get(&fillable) {
                if *state == CellState::Empty {
                    puzzle.insert(fillable, CellState::Filled(colors[2]));
                }
            }
        }

        let mut number_indicators = Vec::new();
        for coord in number_coords {
            let q = coord.0;
            let r = coord.1;
            let s = -q - r;
            if (q == -4 && r == 0) || (q == 4 && r == -4) || (q == 0 && r == 4) {
                continue;
            }
            let hex_coords = HexCoordinates { q, r };
            let new_indicator = if r <= 0 && s >= 0 { // Along q
                let numbers = NumberIndicator::count_color_along_axis(&puzzle, 0, hex_coords, colors[0]);
                NumberIndicator::new(numbers, hex_coords, colors[0], 0)
            } else if r >= 0 && q <= 0 { // Along s
                let numbers = NumberIndicator::count_color_along_axis(&puzzle, 1, hex_coords, colors[1]);
                NumberIndicator::new(numbers, hex_coords, colors[1], 1)
            } else { // Along r
                let numbers = NumberIndicator::count_color_along_axis(&puzzle, 2, hex_coords, colors[2]);
                NumberIndicator::new(numbers, hex_coords, colors[2], 2)
            };
            number_indicators.push(new_indicator)
        }

        Game { cells, colors, shape_indicators, number_indicators }
    }
}

#[derive(Component)]
pub struct GridComponent;

#[derive(Component, Clone, PartialEq, Debug, Copy)]
pub enum CellState{
    Empty,
    Filled(Color),
}

fn draw_grid(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    // https://www.redblobgames.com/grids/hexagons/#basics
    let size = HEXAGON_SIZE / 2.0;

    for cell in &game.cells {
        let coord = cell.0;
        let color = match cell.1 {
            CellState::Empty => Color::srgb(1.0, 1.0, 1.0),
            CellState::Filled(color) => *color // if drawing an already filled grid in the future
        };

        let (x, y) = flat_hex_to_pixel(*coord, size);
        let hex = hexagon_mesh(size - 5.0);
        commands.spawn((
            DespawnOnExit(GameState::Game),
            GridComponent,
            Mesh2d(meshes.add(hex)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, -y, -10.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            *coord,
        )).observe(click_hexagon);
    }
}

fn draw_grid_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    let background_hex = hexagon_mesh(GRID_SIZE + 10.0);
    commands.spawn((
        DespawnOnExit(GameState::Game),
        GridComponent,
        Mesh2d(meshes.add(background_hex)),
        MeshMaterial2d(materials.add(Color::srgba(0.678, 0.847, 0.902, 0.5))),
        Transform::from_xyz(0.0, 0.0, -13.0)
        ));
}

#[derive(Event)]
pub struct ColoredHexagonEvent{
    pub hexagon_coords: HexCoordinates,
}


fn click_hexagon(
    pressed: On<Pointer<Press>>,
    mut game: ResMut<Game>,
    current_color: Res<CurrentColor>,
    hexagons: Query<(&HexCoordinates, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    if pressed.button != PointerButton::Primary {
        return;
    }
    // Checks if the query is good and assigns values from query
    let Ok((coord, mesh_material)) = hexagons.get(pressed.event_target()) else {
        return;
    };

    // Unpacks the cell_state from the hashmap if the coord exists in the hashmap
    if let Some(cell_state) = game.cells.get_mut(coord) {
        *cell_state = match cell_state { // Matches the current state to change color
            CellState::Filled(color) if *color == current_color.color => CellState::Empty, // Matches to the
            _ => CellState::Filled(current_color.color),
        };

        // Finds the new color that was assigned to the cell_state
        let new_color = match *cell_state {
            CellState::Empty => ColorList::color(&ColorList::WHITE),
            CellState::Filled(color) => color,
        }; // Recolors the hexagon
        if let Some(mut material) = materials.get_mut(&mesh_material.0) {
            material.color = new_color;
            commands.trigger(ColoredHexagonEvent{ hexagon_coords: *coord });
        }
    }
}

fn reset_colors(
    button_input: Res<ButtonInput<KeyCode>>,
    mut grid: ResMut<Game>,
    hexagons: Query<(&HexCoordinates, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if button_input.just_pressed(KeyCode::KeyR) {
        let mut changed_colors = false;
        for (coords, mesh_material) in &hexagons { // Loops over all hexagons in the gird
            grid.cells.insert(*coords, CellState::Empty); // Resets the all the colors

            if let Some(mut material) = materials.get_mut(&mesh_material.0) {
                material.color = ColorList::color(&ColorList::WHITE);

                if !changed_colors {
                    changed_colors = true;
                }
            }
        }

        if changed_colors {
            for mut indicator in grid.number_indicators.iter_mut() {
                NumberIndicator::remove_border(&mut indicator, &mut commands);
            }
        }
    }
}

fn exit_to_menu(
    button_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if button_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu)
    }
}

#[derive(Event)]
pub struct PuzzleCorrectEvent;

#[derive(Component)]
enum PuzzleCompleteButtonActions {
    New,
    Quit
}

fn puzzle_correct_handle (
    _ : On<PuzzleCorrectEvent>,
    mut commands: Commands,
    observers: Query<&ObservedBy, With<GridComponent>>,
) {
    for obs in observers.iter() {
        for &observer in obs.get() {
            commands.entity(observer).despawn();
        }
    }
    println!("All correct");

    let button_nodes = Node {
        width: px(350),
        height: px(80),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_font = TextFont {
        font_size: FontSize::Px(33.0),
        ..default()
    };

    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },children![(
            Node {
                width: percent(75),
                height: percent(50),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            children![(
                Text::new("Puzzle Complete"),
                TextFont {
                    font_size: FontSize::Px(67.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::px(0.0, 20.0, 0.0, 0.0),
                    ..default()
                }
            ),(
                Button,
                button_nodes.clone(),
                BackgroundColor(BUTTON_COLOR),
                PuzzleCompleteButtonActions::New,
                children![(
                    Text::new("New Game"),
                    button_font.clone(),
                    TextColor(TEXT_COLOR)
                )]
            ), (
                Button,
                button_nodes,
                BackgroundColor(BUTTON_COLOR),
                PuzzleCompleteButtonActions::Quit,
                children![(
                    Text::new("Quit to desktop"),
                    button_font,
                    TextColor(TEXT_COLOR)
                )]
            )]
        )]
    ));
}

fn click_complete_buttons(
    interaction_query: Query<(&Interaction, &PuzzleCompleteButtonActions), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                PuzzleCompleteButtonActions:: Quit=> {
                    app_exit_writer.write(AppExit::Success);
                }
                PuzzleCompleteButtonActions::New => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}

fn place_shape(
    puzzle: &mut HashMap<HexCoordinates, CellState>,
    mut randomized_coords: Vec<HexCoordinates>,
    shape_type: ShapeType,
    number_to_place: i32,
    rng: &mut ChaCha8Rng,
    color: Color,
) -> i32 {
    let mut number_placed = 0;

    while number_placed < number_to_place && !randomized_coords.is_empty() {
        let Some(random_coord) = randomized_coords.pop() else { break };

        if let Some(random_coord_state) = puzzle.get(&random_coord) {
            if *random_coord_state != CellState::Empty {continue}; // Continue if cells is filled
            let mut rotations = [0, 1, 2, 3, 4, 5]; // Random rotation of place shape
            rotations.shuffle(rng); // Shuffle the rotation to get random rotations

            for rotation in rotations {
                let rotated_shape = ShapeIndicator::rotate_cw_n_times(&shape_type, rotation);
                let mut current_shape = vec!(random_coord); // Start of the shape we try to place

                for i in 1..rotated_shape.len() {
                    let (q, r) = (random_coord.q + rotated_shape[i].q, random_coord.r + rotated_shape[i].r);
                    let current_hexagon = HexCoordinates{ q, r };
                    if let Some(current_hexagon_state) = puzzle.get(&current_hexagon) {
                        if *current_hexagon_state != CellState::Empty { break }; // Current rotation is invalid

                        current_shape.push(current_hexagon); // Builds the shape in the puzzle
                    }
                }

                if current_shape.len() == ShapeType::shape_size(&shape_type) {
                    for hexagon in current_shape {
                        puzzle.insert(hexagon, CellState::Filled(color));
                    }
                    number_placed = number_placed + 1;
                    break; // Stop trying rotations of this place attempt
                }
            }
        }
    }

    number_placed
}
