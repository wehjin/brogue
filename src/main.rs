use bevy::prelude::*;
use bevy_stl::StlPlugin;
use game::plugin::GamePlugin;

mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StlPlugin))
        .add_plugins(GamePlugin)
        .run();
}
