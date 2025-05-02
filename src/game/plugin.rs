use crate::game;
use crate::game::systems::startup::{setup, spawn_items, spawn_rogue, spawn_rooms};
use crate::game::systems::{
    update_walkable_items, update_walkable_directions, handle_rogue_walk,
};
use bevy::prelude::*;
use game::handle_camera_movement;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_rooms, spawn_items, spawn_rogue));
        app.add_systems(
            Update,
            (
                update_walkable_directions,
                update_walkable_items,
                handle_camera_movement,
                handle_rogue_walk,
            ),
        );
    }
}
