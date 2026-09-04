use bevy::prelude::*;
use crate::grid::GridPlugin;
mod grid;
mod color_list;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, GridPlugin));
    app.add_systems(Startup, setup);
    app.run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}