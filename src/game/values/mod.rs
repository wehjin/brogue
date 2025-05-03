use crate::game::values::grid::GridOffset;
use bevy::prelude::Entity;

pub mod grid;
pub mod pack_item;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WalkDestination {
    pub position: GridOffset,
    pub monster: Option<Entity>,
}
