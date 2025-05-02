use crate::game;
use crate::game::systems::setup::{setup, spawn_items, spawn_rogue, spawn_rooms};
use crate::game::systems::stairs::handle_keyboard_ascent;
use crate::game::systems::walk::{
    handle_rogue_walk, update_walkable_directions, update_walkable_items,
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
                handle_keyboard_ascent,
                handle_rogue_walk,
            ),
        );
    }
}
