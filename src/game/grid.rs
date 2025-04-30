use crate::game::constants::{GRID_SOUTH_WEST, TILE_CENTER_OFFSET, TILE_INTERVAL};
use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GridVec {
    pub gx: i32,
    pub gy: i32,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RoguePosition(GridVec);

impl RoguePosition {
    pub fn new(gx: i32, gy: i32) -> Self {
        Self(GridVec { gx, gy })
    }
    pub fn gx_gy(&self) -> (i32, i32) {
        (self.0.gx, self.0.gy)
    }
}

pub struct Tile {
    south_west: Vec3,
}

impl Tile {
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
