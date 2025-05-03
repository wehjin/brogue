use crate::game;
use crate::game::systems::hit::{handle_defeated_monsters, handle_rogue_hit};
use crate::game::systems::setup::{
    spawn_items, spawn_lights_camera, spawn_monsters, spawn_objects, spawn_rogue, spawn_rooms,
};
use crate::game::systems::stairs::handle_keyboard_ascent;
use crate::game::systems::walk::{
    handle_rogue_walk, update_walk_destination, update_walkable_items,
};
use bevy::prelude::*;
use game::handle_camera_movement;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_lights_camera,
                spawn_rooms,
                spawn_items,
                spawn_rogue,
                spawn_monsters,
                spawn_objects,
            ),
        );
        app.add_systems(
            Update,
            (
                handle_camera_movement,
                handle_keyboard_ascent,
                handle_rogue_walk,
                handle_rogue_hit,
                handle_defeated_monsters,
                update_walk_destination,
                update_walkable_items,
            ),
        );
    }
}
