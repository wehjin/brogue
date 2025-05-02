use bevy::prelude::*;
use game::plugin::GamePlugin;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
