use bevy::prelude::*;
use crate::game::{Game, GridComponent};
use crate::hexagon::{HEXAGON_SIZE, hexagon_mesh};

use super::{GameState};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentColor::new(Color::srgb(1.0, 1.0, 1.0)))
            .add_systems(OnEnter(GameState::Game), draw_color_indicators)
            .add_systems(Update, change_active_color.run_if(resource_exists::<Game>))
            .add_observer(change_current_color);
    }
}

#[derive(Resource)]
pub struct CurrentColor {
    pub color: Color
}

impl CurrentColor {
    pub fn new(color: Color) -> Self {
        CurrentColor { color }
    }

    pub fn update(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Component)]
struct ColorIndicator;

#[derive(Component)]
struct ColorSelector {
    color: Color
}

fn draw_color_indicators(
    mut commands: Commands,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    current_color: ResMut<CurrentColor>,
    windows: Query<&mut Window>,
) {
    let size = HEXAGON_SIZE / 2.0;
    for i in 0..4 {
        let hex = hexagon_mesh(size);
        let y = windows.single().unwrap().resolution.height() / 2.0 - size * 2.0;
        if i != 3 { // Takes one of three colors of the grid
            let x = windows.single().unwrap().resolution.width() / 2.0 - size * 2.0 * (4 - i) as f32;
            commands.spawn((
                GridComponent,
                Mesh2d(meshes.add(hex)),
                MeshMaterial2d(materials.add(game.colors[i])),
                Transform::from_xyz(x, y, -10.0),
                ColorSelector{ color: game.colors[i]},
            )).observe(select_color);
        } else {  // This is used to indicate which color is selected
            let x = -windows.single().unwrap().resolution.width() / 2.0 + size * 2.0;
            commands.spawn((
                GridComponent,
                Mesh2d(meshes.add(hex)),
                MeshMaterial2d(materials.add(current_color.color)),
                Transform::from_xyz(x, y, -10.0),
                ColorIndicator,
            ));
        }
    }
}

fn change_active_color(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    current_color: ResMut<CurrentColor>,
    game: Res<Game>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        let mut new_color = game.colors[0];
        if current_color.color != Color::srgb(1.0, 1.0, 1.0) {
            new_color = game.colors[(game.colors.iter().position(|c| c == &current_color.color).unwrap() + 1).rem_euclid(3)]
        };

        commands.trigger(ChangedActiveColorEvent {color: new_color})
    }
}

fn select_color(
    pressed: On<Pointer<Press>>,
    mut commands: Commands,
    hexagon: Query<&ColorSelector>,
) {
    if pressed.button != PointerButton::Primary {
        return;
    }
    let Ok(selector) = hexagon.get(pressed.event_target()) else {
        return;
    };
    commands.trigger(ChangedActiveColorEvent {color: selector.color})
}

#[derive(Event)]
pub struct ChangedActiveColorEvent {
    color: Color,
}

fn change_current_color(
    change_color_event: On<ChangedActiveColorEvent>,
    mut current_color: ResMut<CurrentColor>,
    color_indicator: Query<(&ColorIndicator, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((_indicator, mesh_material)) = color_indicator.single() else {
        return;
    };
    let new_color = change_color_event.color;
    if let Some(mut material) = materials.get_mut(&mesh_material.0) {
        current_color.update(new_color);
        material.color = new_color;
    }
}