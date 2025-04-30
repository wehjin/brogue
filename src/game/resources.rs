use bevy::prelude::Resource;
use std::ops::RangeInclusive;

#[derive(Resource, Clone, Eq, PartialEq, Debug)]
pub struct Room {
    pub south: u32,
    pub north: u32,
    pub west: u32,
    pub east: u32,
}

impl Room {
    pub fn south_to_north(&self) -> RangeInclusive<i32> {
        self.south as i32..=self.north as i32
    }
    pub fn west_to_east(&self) -> RangeInclusive<i32> {
        self.west as i32..=self.east as i32
    }
}
