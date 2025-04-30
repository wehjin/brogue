use crate::game::grid::{GridDirection, GridVec, Tile};
use bevy::prelude::*;

pub struct RoguePlugin;
impl Plugin for RoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rogue);
        app.add_systems(Update, handle_rogue_walk);
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RoguePosition(GridVec);

impl RoguePosition {
    pub fn new(gx: i32, gy: i32) -> Self {
        Self(GridVec { gx, gy })
    }
    pub fn step(&self, direction: GridDirection) -> Self {
        Self(self.0 + direction.to_vec())
    }
}

pub fn spawn_rogue(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = RoguePosition::new(32, 8);
    let tile: Tile = position.0.into();
    let avatar_width = tile.width();
    let avatar_depth = tile.depth() * 0.2;
    let avatar_height = 1.0;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(avatar_width, avatar_height, avatar_depth))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(tile.center()),
        position,
    ));
}

pub fn handle_rogue_walk(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut RoguePosition, &mut Transform)>,
) {
    let direction = if keyboard_input.just_pressed(KeyCode::KeyK) {
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
    };
    if let Some(direction) = direction {
        for (mut pos, mut transform) in query {
            let next_pos = pos.step(direction);
            pos.0 = next_pos.0;
            transform.translation += Tile::step_vec(direction);
        }
    }
}
