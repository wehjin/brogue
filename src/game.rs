use bevy::prelude::*;
use std::ops::RangeInclusive;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
struct GridCoord {
    x: i32,
    y: i32,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Room {
    rows: RangeInclusive<i32>,
    cols: RangeInclusive<i32>,
}

const BIG_ROOM: Room = Room {
    rows: 10..=69,
    cols: 5..=18,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_rooms).chain());
        app.add_systems(Update, handle_camera_movement);
    }
}

const HALF_TILE_INTERVAL: Vec2 = Vec2::new(0.25, 0.5);
const TILE_INTERVAL: Vec2 = Vec2::new(HALF_TILE_INTERVAL.x * 2.0, HALF_TILE_INTERVAL.y * 2.0);
const TILE_CENTER_OFFSET: Vec3 = Vec3::new(HALF_TILE_INTERVAL.x, 0.0, -HALF_TILE_INTERVAL.y);
const HALF_GRID_SIZE: (u32, u32) = (40, 11);
const GRID_WEST: f32 = -(HALF_GRID_SIZE.0 as f32) * TILE_INTERVAL.x;
const GRID_SOUTH: f32 = (HALF_GRID_SIZE.1 as f32) * TILE_INTERVAL.y;
const GRID_SOUTH_WEST: Vec3 = Vec3::new(GRID_WEST, 0.001, GRID_SOUTH);

struct Tile {
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
    pub fn to_center(&self) -> Vec3 {
        self.south_west + TILE_CENTER_OFFSET
    }
    pub fn to_size(&self) -> Vec2 {
        TILE_INTERVAL
    }
}

fn spawn_rooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor base
    let floor_material = materials.add(Color::WHITE);
    const WIDTH:f32 =  TILE_INTERVAL.x * 0.78;
    const HEIGHT:f32 =  TILE_INTERVAL.y * 0.91;
    for gy in 7..15 {
        for gx in 30..50 {
            let tile = Tile::new(gx, gy);
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
                MeshMaterial3d(floor_material.clone()),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_translation(tile.to_center()),
            ));
        }
    }
}

fn setup(
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
    // cube
    let player_tile = Tile::new(32, 8);
    let size = player_tile.to_size();
    let height = size.y * 0.2;
    let center = player_tile.to_center() + Vec3::new(0.0, 0.0, height / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, 1.0, height))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(center),
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
        Transform::from_xyz(0.0, 22.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn handle_camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Camera, &mut Transform)>,
) {
    const STEP: f32 = 0.05;
    let hop = if keyboard_input.pressed(KeyCode::ArrowLeft) {
        Some(Vec3::new(-STEP, 0.0, 0.0))
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        Some(Vec3::new(STEP, 0.0, 0.0))
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        Some(Vec3::new(0.0, 0.0, -STEP))
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        Some(Vec3::new(0.0, 0.0, STEP))
    } else if keyboard_input.pressed(KeyCode::PageUp) {
        Some(Vec3::new(0.0, -STEP, 0.0))
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        Some(Vec3::new(0.0, STEP, 0.0))
    } else {
        None
    };
    if let Some(hop) = hop {
        for (_, mut transform) in query {
            transform.translation += hop;
        }
    }
}
