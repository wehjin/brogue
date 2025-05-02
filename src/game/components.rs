use crate::game::values::grid::GridDirection;
use crate::game::values::pack_item::PackItem;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct Rogue;

#[derive(Component, Debug, Default)]
pub struct Pack {
    pub items: Vec<PackItem>,
}
impl Pack {
    pub fn find_item(&self, search: PackItem) -> Option<&PackItem> {
        self.items.iter().find(|&item| *item == search)
    }
}

#[derive(Component)]
pub struct Amulet;

#[derive(Component)]
pub struct GroundItem;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TileType {
    Floor,
    Stairs,
}

#[derive(Component, Clone, Eq, PartialEq, Debug, Default)]
pub struct WalkableDirections(pub HashSet<GridDirection>);

#[derive(Component, Clone, Eq, PartialEq, Debug, Default)]
pub struct WalkableItems(pub HashMap<GridDirection, Entity>);

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RoomBounds {
    pub south: u32,
    pub north: u32,
    pub west: u32,
    pub east: u32,
}

impl RoomBounds {
    pub fn south_to_north(&self) -> RangeInclusive<i32> {
        self.south as i32..=self.north as i32
    }
    pub fn west_to_east(&self) -> RangeInclusive<i32> {
        self.west as i32..=self.east as i32
    }
}
