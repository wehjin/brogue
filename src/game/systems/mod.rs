use crate::game::components::{Rogue, TileType, WalkableDirections};
use crate::game::grid::{GridDirection, GridOffset, Tile};
use bevy::prelude::*;
use std::collections::HashSet;

pub mod startup;

pub fn compute_walkable_directions(
    rogue: Single<(&GridOffset, &mut WalkableDirections), With<Rogue>>,
    tiles: Query<(&GridOffset, &TileType)>,
) {
    let (rogue_offset, mut walkable_directions) = rogue.into_inner();
    let mut new_directions = HashSet::new();
    for (tile_offset, tile_type) in tiles.iter() {
        match tile_type {
            TileType::Floor => {
                let tile_delta = *tile_offset - *rogue_offset;
                if let Ok(direction) = GridDirection::try_from(tile_delta) {
                    new_directions.insert(direction);
                }
            }
        }
    }
    if new_directions != walkable_directions.0 {
        walkable_directions.0 = new_directions;
    }
}

pub fn handle_rogue_walk(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rogue: Single<(&mut GridOffset, &mut Transform, &WalkableDirections), With<Rogue>>,
) {
    if let Some(direction) = check_walk_keys(keyboard_input) {
        let (mut grid_offset, mut transform, walkable) = rogue.into_inner();
        if walkable.0.contains(&direction) {
            *grid_offset += GridOffset::from(direction);
            transform.translation += Tile::step_vec(direction);
        }
    }
}

fn check_walk_keys(keyboard_input: Res<ButtonInput<KeyCode>>) -> Option<GridDirection> {
    if keyboard_input.just_pressed(KeyCode::KeyK) {
        Some(GridDirection::North)
    } else if keyboard_input.just_pressed(KeyCode::KeyJ) {
        Some(GridDirection::South)
    } else if keyboard_input.just_pressed(KeyCode::KeyL) {
        Some(GridDirection::East)
    } else if keyboard_input.just_pressed(KeyCode::KeyH) {
        Some(GridDirection::West)
    } else if keyboard_input.just_pressed(KeyCode::KeyU) {
        Some(GridDirection::NorthEast)
    } else if keyboard_input.just_pressed(KeyCode::KeyY) {
        Some(GridDirection::NorthWest)
    } else if keyboard_input.just_pressed(KeyCode::KeyN) {
        Some(GridDirection::SouthEast)
    } else if keyboard_input.just_pressed(KeyCode::KeyB) {
        Some(GridDirection::SouthWest)
    } else {
        None
    }
}
