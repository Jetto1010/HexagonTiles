use bevy::prelude::*;
use crate::grid::GridPlugin;
use crate::menu::menu_plugin;

mod grid;
mod color_list;
mod color_selection;
mod numbers;
mod shapes;
mod hexagon;
mod puzzle;
mod menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((menu_plugin, GridPlugin))
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}