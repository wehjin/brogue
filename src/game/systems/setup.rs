use crate::game::components::{
    Amulet, GroundItem, Pack, Rogue, RoomBounds, TileType, WalkableDirections, WalkableItems,
};
use crate::game::constants::TILE_INTERVAL;
use crate::game::values::grid::{GridOffset, Tile};
use bevy::color::palettes::css;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::prelude::*;

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
        Pack::default(),
        WalkableDirections::default(),
        WalkableItems::default(),
        Mesh3d(meshes.add(Cuboid::new(avatar_width, avatar_height, avatar_depth))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(tile.center()),
    ));
}
pub fn spawn_items(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tile = Tile::new(40, 11);
    let amulet_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("amulet.png")),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        Amulet,
        GroundItem,
        tile.grid_position,
        Mesh3d(meshes.add(Rectangle::new(TILE_INTERVAL.x, TILE_INTERVAL.y))),
        MeshMaterial3d(amulet_material),
        Transform::from_translation(tile.center() + Vec3::new(0.0, 0.001, 0.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(0.5)),
    ));
}
pub fn spawn_rooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_material = materials.add(Color::WHITE);
    let stairs_material = materials.add(StandardMaterial::from_color(css::GOLDENROD));

    // layout
    let room_bounds = RoomBounds {
        south: 7,
        north: 12,
        west: 30,
        east: 50,
    };
    let stairs_position = GridOffset::new(48, 10);

    const WIDTH: f32 = TILE_INTERVAL.x * 0.78;
    const HEIGHT: f32 = TILE_INTERVAL.y * 0.91;
    for gy in room_bounds.south_to_north() {
        for gx in room_bounds.west_to_east() {
            let tile = Tile::new(gx, gy);

            let (material, tile_type) = if tile.grid_position == stairs_position {
                (stairs_material.clone(), TileType::Stairs)
            } else {
                (floor_material.clone(), TileType::Floor)
            };
            commands.spawn((
                tile_type,
                tile.grid_position,
                Mesh3d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
                MeshMaterial3d(material),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_translation(tile.center()),
            ));
        }
    }

    // stairs
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // level
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(40.0, 22.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0x44, 0x55, 0x66))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -0.010, 0.0)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        //Transform::from_xyz(4.0, 8.0, 4.0),
        Transform::from_xyz(0.0, 8.0, 3.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        Transform::from_xyz(0.0, 26.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
