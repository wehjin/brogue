use crate::game::components::{
    GroundItem, Pack, Rogue, TileType, WalkableDirections, WalkableItems,
};
use crate::game::values::grid::{GridDirection, GridOffset, Tile};
use crate::game::values::pack_item::PackItem;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Entity, KeyCode, Query, Res, Single, Transform, With};
use std::collections::{HashMap, HashSet};

pub fn update_walkable_directions(
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

pub fn update_walkable_items(
    rogue: Single<(&GridOffset, &mut WalkableItems), With<Rogue>>,
    items: Query<(Entity, &GridOffset), With<GroundItem>>,
) {
    let (rogue_offset, mut touchable_items) = rogue.into_inner();
    let mut new_items = HashMap::new();
    for (item_entity, item_offset) in items.iter() {
        let delta = *item_offset - *rogue_offset;
        let grid_direction = GridDirection::try_from(delta);
        if let Ok(grid_direction) = grid_direction {
            new_items.insert(grid_direction, item_entity);
        }
    }
    if new_items != touchable_items.0 {
        touchable_items.0 = new_items;
    }
}

pub fn handle_rogue_walk(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rogue: Single<
        (
            &mut GridOffset,
            &mut Transform,
            &WalkableDirections,
            &WalkableItems,
            &mut Pack,
        ),
        With<Rogue>,
    >,
    mut commands: Commands,
) {
    if let Some(direction) = check_walk_keys(keyboard_input) {
        let (mut rogue_offset, mut transform, directions, items, mut pack) = rogue.into_inner();
        if directions.0.contains(&direction) {
            if let Some(item_entity) = items.0.get(&direction) {
                commands.entity(*item_entity).despawn();
                pack.items.push(PackItem::Amulet);
                println!("{:?}", pack.as_ref());
            }
            *rogue_offset += GridOffset::from(direction);
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
