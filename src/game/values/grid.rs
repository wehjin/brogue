use crate::game::constants::{GRID_SOUTH_WEST, TILE_CENTER_OFFSET, TILE_INTERVAL};
use bevy::prelude::*;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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

impl GridDirection {}

impl TryFrom<GridOffset> for GridDirection {
    type Error = &'static str;

    fn try_from(value: GridOffset) -> std::result::Result<Self, Self::Error> {
        match value {
            GridOffset::NORTH => Ok(GridDirection::North),
            GridOffset::NORTH_EAST => Ok(GridDirection::NorthEast),
            GridOffset::EAST => Ok(GridDirection::East),
            GridOffset::SOUTH_EAST => Ok(GridDirection::SouthEast),
            GridOffset::SOUTH => Ok(GridDirection::South),
            GridOffset::SOUTH_WEST => Ok(GridDirection::SouthWest),
            GridOffset::WEST => Ok(GridDirection::West),
            GridOffset::NORTH_WEST => Ok(GridDirection::NorthWest),
            _ => Err("invalid offset"),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GridOffset {
    pub gx: i32,
    pub gy: i32,
}
impl GridOffset {
    pub const fn new(gx: i32, gy: i32) -> Self {
        Self { gx, gy }
    }
    pub fn _len(&self) -> u32 {
        self.gx.unsigned_abs().max(self.gy.unsigned_abs())
    }
}

impl From<GridDirection> for GridOffset {
    fn from(value: GridDirection) -> Self {
        match value {
            GridDirection::North => GridOffset::NORTH,
            GridDirection::NorthEast => GridOffset::NORTH_EAST,
            GridDirection::East => GridOffset::EAST,
            GridDirection::SouthEast => GridOffset::SOUTH_EAST,
            GridDirection::South => GridOffset::SOUTH,
            GridDirection::SouthWest => GridOffset::SOUTH_WEST,
            GridDirection::West => GridOffset::WEST,
            GridDirection::NorthWest => GridOffset::NORTH_WEST,
        }
    }
}

impl Add for GridOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            gx: self.gx + rhs.gx,
            gy: self.gy + rhs.gy,
        }
    }
}

impl AddAssign for GridOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.gx += rhs.gx;
        self.gy += rhs.gy;
    }
}

impl Sub for GridOffset {
    type Output = GridOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        GridOffset {
            gx: self.gx - rhs.gx,
            gy: self.gy - rhs.gy,
        }
    }
}

pub struct Tile {
    pub grid_position: GridOffset,
    pub south_west: Vec3,
}

impl From<GridOffset> for Tile {
    fn from(value: GridOffset) -> Self {
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
            grid_position: GridOffset::new(gx, gy),
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
