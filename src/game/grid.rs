use crate::game::constants::{GRID_SOUTH_WEST, TILE_CENTER_OFFSET, TILE_INTERVAL};
use bevy::prelude::*;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GridDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl GridDirection {
    pub fn to_vec(&self) -> GridVec {
        match self {
            GridDirection::North => GridVec::new(1, 0),
            GridDirection::NorthEast => GridVec::new(1, 1),
            GridDirection::East => GridVec::new(0, 1),
            GridDirection::SouthEast => GridVec::new(-1, 1),
            GridDirection::South => GridVec::new(-1, 0),
            GridDirection::SouthWest => GridVec::new(-1, -1),
            GridDirection::West => GridVec::new(0, -1),
            GridDirection::NorthWest => GridVec::new(1, -1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GridVec {
    pub gx: i32,
    pub gy: i32,
}
impl Add for GridVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            gx: self.gx + rhs.gx,
            gy: self.gy + rhs.gy,
        }
    }
}

impl GridVec {
    pub fn new(gx: i32, gy: i32) -> Self {
        Self { gx, gy }
    }
}

pub struct Tile {
    south_west: Vec3,
}

impl From<GridVec> for Tile {
    fn from(value: GridVec) -> Self {
        Tile::new(value.gx, value.gy)
    }
}

impl Tile {
    pub fn step_vec(direction: GridDirection) -> Vec3 {
        match direction {
            GridDirection::North => Vec3::new(0.0, 0.0, -TILE_INTERVAL.y),
            GridDirection::NorthEast => Vec3::new(TILE_INTERVAL.x, 0.0, -TILE_INTERVAL.y),
            GridDirection::East => Vec3::new(TILE_INTERVAL.x, 0.0, 0.0),
            GridDirection::SouthEast => Vec3::new(TILE_INTERVAL.x, 0.0, TILE_INTERVAL.y),
            GridDirection::South => Vec3::new(0.0, 0.0, TILE_INTERVAL.y),
            GridDirection::SouthWest => Vec3::new(-TILE_INTERVAL.x, 0.0, TILE_INTERVAL.y),
            GridDirection::West => Vec3::new(-TILE_INTERVAL.x, 0.0, 0.0),
            GridDirection::NorthWest => Vec3::new(-TILE_INTERVAL.x, 0.0, -TILE_INTERVAL.y),
        }
    }

    pub fn new(gx: i32, gy: i32) -> Self {
        let offset = Vec3::new(
            (gx as f32) * TILE_INTERVAL.x,
            0.0,
            -(gy as f32) * TILE_INTERVAL.y,
        );
        Tile {
            south_west: GRID_SOUTH_WEST + offset,
        }
    }
    pub fn center(&self) -> Vec3 {
        self.south_west + TILE_CENTER_OFFSET
    }
    pub fn depth(&self) -> f32 {
        TILE_INTERVAL.y
    }
    pub fn width(&self) -> f32 {
        TILE_INTERVAL.x
    }
}
