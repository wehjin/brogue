use crate::game::grid::GridDirection;
use bevy::prelude::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct Rogue;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TileType {
    Floor,
}

#[derive(Component, Clone, Eq, PartialEq, Debug, Default)]
pub struct WalkableDirections(pub HashSet<GridDirection>);

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
