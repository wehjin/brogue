use crate::game::constants::INIT_HP;
use crate::game::values::grid::GridDirection;
use crate::game::values::pack_item::PackItem;
use crate::game::values::WalkDestination;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct Rogue;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RogueHealth {
    pub current_hp: isize,
    pub max_hp: isize,
}

impl Default for RogueHealth {
    fn default() -> Self {
        Self {
            current_hp: INIT_HP,
            max_hp: INIT_HP,
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RogueBuffs {
    pub xp_points: isize,
    pub xp_level: isize,
    pub rings_xp_level_boost: isize,
    pub rings_on_hands: isize,
    pub str_current: isize,
    pub str_max: isize,
    pub rings_add_strength: isize,
}

impl RogueBuffs {
    pub fn xp_level_with_ring_boost(&self) -> isize {
        self.xp_level + self.rings_xp_level_boost
    }

    pub fn damage_for_strength(&self) -> isize {
        let strength = self.str_current + self.rings_add_strength;
        match strength as usize {
            0..=6 => strength - 5,
            7..=14 => 1,
            15..=17 => 3,
            18..=20 => 5,
            21..=21 => 6,
            22..=30 => 7,
            _ => 8,
        }
    }
}

impl Default for RogueBuffs {
    fn default() -> Self {
        Self {
            xp_points: 0,
            xp_level: 1,
            rings_xp_level_boost: 0,
            rings_on_hands: 0,
            str_current: 16,
            str_max: 16,
            rings_add_strength: 0,
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct WeaponBuffs {
    pub hit_enchant: isize,
    pub d_enchant: isize,
}

impl Default for WeaponBuffs {
    fn default() -> Self {
        Self {
            hit_enchant: 0,
            d_enchant: 0,
        }
    }
}

#[derive(Component)]
pub struct RogueHitTarget;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum MonsterType {
    Aquator,
}
impl MonsterType {
    pub fn starting_hp(&self) -> isize {
        match self {
            MonsterType::Aquator => 25,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            MonsterType::Aquator => "aquator",
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct MonsterHealth {
    pub hp_to_kill: isize,
}

impl From<MonsterType> for MonsterHealth {
    fn from(value: MonsterType) -> Self {
        Self {
            hp_to_kill: value.starting_hp(),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Weapon;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum WeaponType {
    Mace,
}
impl WeaponType {
    pub fn damage(&self) -> (isize, isize) {
        match self {
            WeaponType::Mace => (2, 3),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RogueEquipped;

#[derive(Component, Debug, Default)]
pub struct RoguePack {
    pub items: Vec<PackItem>,
}

impl RoguePack {
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
pub struct WalkDestinations(pub HashMap<GridDirection, WalkDestination>);

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
