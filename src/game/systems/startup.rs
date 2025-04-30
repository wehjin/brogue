use crate::game::components::{Rogue, RoomBounds, TileType, WalkableDirections};
use crate::game::grid::{GridOffset, Tile};
use bevy::prelude::*;
use crate::game::constants::TILE_INTERVAL;

pub fn spawn_rogue(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_offset = GridOffset::new(32, 8);
    let tile: Tile = grid_offset.into();
    let avatar_width = tile.width();
    let avatar_depth = tile.depth() * 0.2;
    let avatar_height = 1.0;
    commands.spawn((
        Rogue,
        grid_offset,
        WalkableDirections::default(),
        Mesh3d(meshes.add(Cuboid::new(avatar_width, avatar_height, avatar_depth))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(tile.center()),
    ));
}

pub fn spawn_rooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bounds = RoomBounds {
        south: 7,
        north: 12,
        west: 30,
        east: 50,
    };
    // floor base
    let floor_material = materials.add(Color::WHITE);
    const WIDTH: f32 = TILE_INTERVAL.x * 0.78;
    const HEIGHT: f32 = TILE_INTERVAL.y * 0.91;
    for gy in bounds.south_to_north() {
        for gx in bounds.west_to_east() {
            let tile = Tile::new(gx, gy);
            commands.spawn((
                TileType::Floor,
                Mesh3d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
                MeshMaterial3d(floor_material.clone()),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_translation(tile.center()),
                tile.grid_offset,
            ));
        }
    }
}