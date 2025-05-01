use crate::game::systems::compute_walkable_directions;
use crate::game::systems::startup::spawn_items;
use bevy::prelude::*;
use startup::setup;
use systems::startup::spawn_rogue;
use systems::startup::spawn_rooms;
use systems::{handle_rogue_walk, startup};

pub mod components;
pub mod constants;
pub mod grid;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup, spawn_rooms, spawn_items, spawn_rogue),
        );
        app.add_systems(
            Update,
            (
                compute_walkable_directions,
                handle_camera_movement,
                handle_rogue_walk,
            ),
        );
    }
}

fn handle_camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Camera, &mut Transform)>,
) {
    const STEP: f32 = 0.05;
    let hop = if keyboard_input.pressed(KeyCode::ArrowLeft) {
        Some(Vec3::new(-STEP, 0.0, 0.0))
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        Some(Vec3::new(STEP, 0.0, 0.0))
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        Some(Vec3::new(0.0, 0.0, -STEP))
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        Some(Vec3::new(0.0, 0.0, STEP))
    } else if keyboard_input.pressed(KeyCode::PageUp) {
        Some(Vec3::new(0.0, -STEP, 0.0))
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        Some(Vec3::new(0.0, STEP, 0.0))
    } else {
        None
    };
    if let Some(hop) = hop {
        for (_, mut transform) in query {
            transform.translation += hop;
        }
    }
}
