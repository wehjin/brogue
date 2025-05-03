use crate::game::components::{
    GroundItem, MonsterType, Rogue, RogueHitTarget, RoguePack, TileType, WalkDestinations,
    WalkableItems,
};
use crate::game::values::grid::{GridDirection, GridOffset, Tile};
use crate::game::values::pack_item::PackItem;
use crate::game::values::WalkDestination;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Entity, KeyCode, Query, Res, Single, Transform, With};
use std::collections::HashMap;

pub fn update_walk_destination(
    query_rogue: Single<(&GridOffset, &mut WalkDestinations), With<Rogue>>,
    query_tiles: Query<(&GridOffset, &TileType)>,
    query_monsters: Query<(&GridOffset, Entity), With<MonsterType>>,
) {
    let monster_entities = query_monsters.iter().collect::<HashMap<_, _>>();
    let (rogue_offset, mut walk_destinations) = query_rogue.into_inner();
    let mut new_destinations = HashMap::new();
    for (tile_offset, tile_type) in query_tiles.iter() {
        match tile_type {
            TileType::Floor | TileType::Stairs => {
                if let Ok(direction) = GridDirection::try_from(*tile_offset - *rogue_offset) {
                    let destination = WalkDestination {
                        position: *tile_offset,
                        monster: monster_entities.get(tile_offset).cloned(),
                    };
                    new_destinations.insert(direction, destination);
                }
            }
        }
    }
    if new_destinations != walk_destinations.0 {
        walk_destinations.0 = new_destinations;
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
    query_rogue: Single<
        (
            &mut GridOffset,
            &mut Transform,
            &WalkDestinations,
            &WalkableItems,
            &mut RoguePack,
        ),
        With<Rogue>,
    >,
    mut commands: Commands,
) {
    if let Some(walk_direction) = walk_direction(keyboard_input) {
        let (mut rogue_position, mut transform, walk_destination, items, mut pack) =
            query_rogue.into_inner();
        if let Some(walk_destination) = walk_destination.0.get(&walk_direction) {
            if let Some(monster_entity) = walk_destination.monster {
                commands.entity(monster_entity).insert(RogueHitTarget);
            } else {
                if let Some(item_entity) = items.0.get(&walk_direction) {
                    commands.entity(*item_entity).despawn();
                    pack.items.push(PackItem::Amulet);
                }
                *rogue_position += GridOffset::from(walk_direction);
                transform.translation += Tile::step_vec(walk_direction);
            }
        }
    }
}

fn walk_direction(keyboard_input: Res<ButtonInput<KeyCode>>) -> Option<GridDirection> {
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
