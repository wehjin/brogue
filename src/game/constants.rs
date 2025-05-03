use crate::game::values::grid::GridOffset;
use bevy::math::{Vec2, Vec3};

pub const INIT_HP: isize = 12;

pub const HALF_TILE_INTERVAL: Vec2 = Vec2::new(0.25, 0.5);
pub const TILE_INTERVAL: Vec2 = Vec2::new(HALF_TILE_INTERVAL.x * 2.0, HALF_TILE_INTERVAL.y * 2.0);
pub const TILE_CENTER_OFFSET: Vec3 = Vec3::new(HALF_TILE_INTERVAL.x, 0.0, -HALF_TILE_INTERVAL.y);
const HALF_GRID_SIZE: (u32, u32) = (40, 11);
const GRID_WEST: f32 = -(HALF_GRID_SIZE.0 as f32) * TILE_INTERVAL.x;
const GRID_SOUTH: f32 = (HALF_GRID_SIZE.1 as f32) * TILE_INTERVAL.y;
pub const GRID_SOUTH_WEST: Vec3 = Vec3::new(GRID_WEST, 0.001, GRID_SOUTH);

impl GridOffset {
    pub const NORTH: Self = Self::new(0, 1);
    pub const NORTH_EAST: Self = Self::new(1, 1);
    pub const EAST: Self = Self::new(1, 0);
    pub const SOUTH_EAST: Self = Self::new(1, -1);
    pub const SOUTH: Self = Self::new(0, -1);
    pub const SOUTH_WEST: Self = Self::new(-1, -1);
    pub const WEST: Self = Self::new(-1, 0);
    pub const NORTH_WEST: Self = Self::new(-1, 1);
}
