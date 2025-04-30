use crate::game::systems::compute_walkable_directions;
use bevy::prelude::*;
use systems::handle_rogue_walk;
use systems::startup::spawn_rogue;
use systems::startup::spawn_rooms;

pub mod components;
pub mod constants;
pub mod grid;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_rooms, spawn_rogue));
        app.add_systems(
            Update,
            (
                compute_walkable_directions,
                handle_camera_movement,
                handle_rogue_walk,
            ),
        );
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
