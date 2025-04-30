use self::grid::Tile;
use crate::game::grid::RoguePosition;
use crate::game::resources::Room;
use bevy::prelude::*;
use constants::TILE_INTERVAL;

mod constants;
pub mod grid;
mod resources;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Room {
            south: 7,
            north: 12,
            west: 30,
            east: 50,
        });
        app.add_systems(Startup, (setup, spawn_rooms, spawn_rogue).chain());
        app.add_systems(Update, handle_camera_movement);
    }
}

fn spawn_rooms(
    room: Res<Room>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor base
    let floor_material = materials.add(Color::WHITE);
    const WIDTH: f32 = TILE_INTERVAL.x * 0.78;
    const HEIGHT: f32 = TILE_INTERVAL.y * 0.91;
    for gy in room.south_to_north() {
        for gx in room.west_to_east() {
            let tile = Tile::new(gx, gy);
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
                MeshMaterial3d(floor_material.clone()),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_translation(tile.center()),
            ));
        }
    }
}

fn spawn_rogue(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = RoguePosition::new(32, 8);
    let (gx, gy) = position.gx_gy();
    let tile = Tile::new(gx, gy);
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
        Transform::from_xyz(0.0, 26.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
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
